use std::cmp::max;
use std::net::UdpSocket;
use std::ops::{Add, Sub};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use cheetah_matches_relay_common::room::{RoomId, UserId};

use crate::room::template::config::{RoomTemplate, UserTemplate};
use crate::rooms::{RegisterUserError, Rooms};
use crate::server::dump::ServerDump;
use crate::server::udp::UDPServer;
use crate::server::Request::TimeOffset;

pub mod dump;
pub mod udp;

pub struct RelayServer {
	handler: Option<JoinHandle<()>>,
	sender: Sender<Request>,
	halt_signal: Arc<AtomicBool>,
	pub created_room_counter: usize,
}

enum Request {
	RegisterRoom(RoomTemplate, Sender<RoomId>),
	RegisterUser(RoomId, UserTemplate, Sender<Result<UserId, RegisterUserError>>),
	///
	/// Смещение текущего времени для тестирования
	///
	TimeOffset(Duration),

	///
	/// Скопировать состояние сервера для отладки
	///
	Dump(Sender<ServerDump>),
}

#[derive(Debug)]
pub enum RegisterRoomRequestError {
	ChannelError(RecvTimeoutError),
}

#[derive(Debug)]
pub enum RegisterUserRequestError {
	ChannelError(RecvTimeoutError),
	Error(RegisterUserError),
}

impl Drop for RelayServer {
	fn drop(&mut self) {
		self.halt_signal.store(true, Ordering::Relaxed);
	}
}

impl RelayServer {
	pub fn new(socket: UdpSocket) -> Self {
		let (sender, receiver) = std::sync::mpsc::channel();
		let halt_signal = Arc::new(AtomicBool::new(false));
		let cloned_halt_signal = halt_signal.clone();
		let handler = thread::Builder::new()
			.name(format!("server({:?})", socket.local_addr().unwrap()))
			.spawn(move || {
				ServerThread::new(socket, receiver, halt_signal).run();
			})
			.unwrap();
		Self {
			handler: Option::Some(handler),
			sender,
			halt_signal: cloned_halt_signal,
			created_room_counter: 0,
		}
	}

	pub fn get_halt_signal(&self) -> Arc<AtomicBool> {
		self.halt_signal.clone()
	}

	pub fn register_room(&mut self, template: RoomTemplate) -> Result<RoomId, RegisterRoomRequestError> {
		let (sender, receiver) = std::sync::mpsc::channel();
		self.sender.send(Request::RegisterRoom(template, sender)).unwrap();
		self.created_room_counter += 1;
		match receiver.recv_timeout(Duration::from_millis(100)) {
			Ok(room_id) => {
				log::info!("[server] create room({:?})", room_id);
				Result::Ok(room_id)
			}
			Err(e) => {
				log::error!("[server] fail create room");
				Result::Err(RegisterRoomRequestError::ChannelError(e))
			}
		}
	}

	pub fn register_user(&mut self, room_id: RoomId, template: UserTemplate) -> Result<UserId, RegisterUserRequestError> {
		let (sender, receiver) = std::sync::mpsc::channel();
		self.sender.send(Request::RegisterUser(room_id, template.clone(), sender)).unwrap();
		match receiver.recv_timeout(Duration::from_millis(100)) {
			Ok(r) => match r {
				Ok(user_id) => {
					log::info!("[server] create user({:?}) in room ({:?})", user_id, room_id);
					Result::Ok(user_id)
				}
				Err(e) => {
					log::error!(
						"[server] fail create user ({:?}) in room ({:?}) with error {:?}",
						template.private_key,
						room_id,
						e
					);
					Result::Err(RegisterUserRequestError::Error(e))
				}
			},
			Err(e) => {
				log::error!(
					"[server] fail create user ({:?}) in room ({:?}) with error {:?}",
					template.private_key,
					room_id,
					e
				);
				Result::Err(RegisterUserRequestError::ChannelError(e))
			}
		}
	}

	pub fn set_time_offset(&self, duration: Duration) {
		self.sender.send(TimeOffset(duration)).unwrap();
	}

	pub fn join(&mut self) {
		self.handler.take().unwrap().join().unwrap();
	}

	pub fn dump(&self) -> Result<ServerDump, ()> {
		let (sender, receiver) = std::sync::mpsc::channel();
		self.sender.send(Request::Dump(sender)).unwrap();
		receiver.recv().map_err(|_| ())
	}
}

struct ServerThread {
	udp_server: UDPServer,
	rooms: Rooms,
	receiver: Receiver<Request>,
	max_cycle_time: u128,
	avg_cycle_time: u128,
	halt_signal: Arc<AtomicBool>,
	time_offset: Option<Duration>,
}

impl ServerThread {
	pub fn new(socket: UdpSocket, receiver: Receiver<Request>, halt_signal: Arc<AtomicBool>) -> Self {
		Self {
			udp_server: UDPServer::new(socket).unwrap(),
			rooms: Rooms::new(),
			receiver,
			max_cycle_time: 0,
			avg_cycle_time: 0,
			halt_signal,
			time_offset: None,
		}
	}

	pub fn run(&mut self) {
		while !self.halt_signal.load(Ordering::Relaxed) {
			let mut now = Instant::now();
			if let Some(time_offset) = self.time_offset {
				now = now.add(time_offset);
			}
			self.udp_server.cycle(&mut self.rooms, &now);
			self.rooms.cycle(&now);
			self.do_request();
			self.statistics(now);
		}
	}

	fn do_request(&mut self) {
		while let Ok(request) = self.receiver.try_recv() {
			match request {
				Request::RegisterRoom(template, sender) => {
					let listener = self.udp_server.get_room_user_listener();
					let result = self.rooms.create_room(template.clone(), vec![listener]);
					match sender.send(result) {
						Ok(_) => {}
						Err(e) => {
							log::error!("[Request::RegisterRoom] error send response {:?}", e)
						}
					}
				}
				Request::RegisterUser(room_id, user_template, sender) => {
					let register_user_result = self.rooms.register_user(room_id, user_template);
					match sender.send(register_user_result) {
						Ok(_) => {}
						Err(e) => {
							log::error!("[Request::RegisterUser] error send response {:?}", e)
						}
					}
				}
				TimeOffset(time_offset) => {
					self.time_offset = Option::Some(time_offset);
				}
				Request::Dump(sender) => {
					sender.send(ServerDump::from(&*self)).unwrap();
				}
			}
		}
	}

	fn statistics(&mut self, start_instant: Instant) {
		let end_instant = Instant::now();
		let duration = end_instant.sub(start_instant).as_millis();
		if duration < 2 {
			thread::sleep(Duration::from_millis(1));
		}
		if self.avg_cycle_time == 0 {
			self.avg_cycle_time = duration;
		} else {
			self.avg_cycle_time = (self.avg_cycle_time + duration) / 2;
		}
		self.max_cycle_time = max(self.max_cycle_time, duration);
	}
}

#[cfg(test)]
mod test {
	use cheetah_matches_relay_common::network::bind_to_free_socket;

	use crate::room::template::config::RoomTemplate;
	use crate::server::RelayServer;

	#[test]
	fn should_increment_created_room_count() {
		let mut server = RelayServer::new(bind_to_free_socket().unwrap().0);
		server.register_room(RoomTemplate::default()).unwrap();
		assert_eq!(server.created_room_counter, 1);
	}
}
