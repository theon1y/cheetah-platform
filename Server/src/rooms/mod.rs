use std::collections::{HashMap, HashSet};
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Sender, SendError};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use cheetah_relay_common::network::hash::HashValue;

use crate::room::request::RoomRequest;
use crate::room::thread::RoomThread;

///
/// Реестр комнат
///
pub struct Rooms {
	auto_create_rooms_and_clients: bool,
	registry: Arc<Mutex<HashMap<HashValue, RoomThreadController>>>,
}

///
/// каналы для управления Room
///
pub struct RoomThreadController {
	pub sender: Sender<RoomRequest>,
	pub handle: JoinHandle<()>,
}

pub enum SendRoomRequestError {
	RoomNotFound,
	SendError(SendError<RoomRequest>),
}


impl Rooms {
	pub fn new(auto_create_rooms_and_clients: bool) -> Self {
		Rooms {
			auto_create_rooms_and_clients,
			registry: Default::default(),
		}
	}
	
	
	pub fn create_room(&mut self, room_hash: &HashValue) {
		let (sender, receiver) = mpsc::channel();
		
		let cloned_room_hash = room_hash.clone();
		let auto_create_rooms_and_clients = self.auto_create_rooms_and_clients;
		let handle = thread::spawn(move || {
			let mut room_cycle = RoomThread::new(cloned_room_hash, auto_create_rooms_and_clients, receiver);
			room_cycle.run();
		});
		
		
		let registry = &*self.registry.clone();
		let mut registry = registry.lock().unwrap();
		registry.insert(room_hash.clone(), RoomThreadController {
			sender,
			handle,
		});
	}
	
	pub fn send_room_request(&mut self, room_hash: &HashValue, request: RoomRequest) -> Result<(), SendRoomRequestError> {
		let registry = &*self.registry.clone();
		let registry = registry.lock().unwrap();
		let room = registry.get(room_hash);
		match room {
			None => {
				if self.auto_create_rooms_and_clients {
					log::trace!("Auto create room {:?}", room_hash);
					drop(registry);
					self.create_room(room_hash);
					self.send_room_request(room_hash, request)
				} else {
					Result::Err(SendRoomRequestError::RoomNotFound)
				}
			}
			Some(room) => {
				match room.sender.send(request) {
					Ok(_) => {
						Result::Ok(())
					}
					Err(error) => {
						Result::Err(SendRoomRequestError::SendError(error))
					}
				}
			}
		}
	}
	
	pub fn destroy_room(&mut self, room_hash: &HashValue) {
		self.send_room_request(room_hash, RoomRequest::Destroy).ok().unwrap();
		self.registry.lock().unwrap().remove(room_hash);
	}
	
	pub fn collect_rooms_hashes<F>(&self, mut collector: F) where F: FnMut(&HashValue) -> () {
		let registry = &mut self.registry.lock().unwrap();
		for hash in registry.keys() {
			collector(hash);
		}
	}
	
	///
	/// Закрыть все комнаты (включая сетевые коннекты)
	///
	pub fn close_all_rooms(&mut self) {
		let mut rooms = HashSet::new();
		{
			let registry = &mut self.registry.lock().unwrap();
			for room_hash in registry.keys() {
				rooms.insert(room_hash.clone());
			}
		}
		for room in &rooms {
			self.send_room_request(room, RoomRequest::Destroy).ok().unwrap();
		}
		
		let mut registry = self.registry.lock().unwrap();
		for room in &rooms {
			let controller = registry.remove(room).unwrap();
			controller.handle.join();
		}
	}
}