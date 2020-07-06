use std::cell::RefCell;
use std::rc::Rc;
use cheetah_relay::room::clients::{Client, Clients};
use cheetah_relay::room::listener::RoomListener;
use cheetah_relay::room::objects::object::GameObject;
use cheetah_relay::room::objects::Objects;
use cheetah_relay::room::Room;
use cheetah_relay_common::constants::GroupType;
use cheetah_relay_common::network::hash::HashValue;
use cheetah_relay_common::room::access::AccessGroups;

use crate::unit::relay::room::room::room_stub;

pub mod room;
pub mod objects;
pub mod clients;
pub mod groups;
pub mod listener;

pub fn setup_and_two_client() -> (Room, Rc<Client>, Rc<Client>) {
	let mut room = room_stub();
	let first_client = setup_client(&mut room, "CLIENT-A", 0b100001);
	let second_client = setup_client(&mut room, "CLIENT-B", 0b100001);
	(room, first_client, second_client)
}

pub fn setup_client(room: &mut Room, client_hash: &str, group: GroupType) -> Rc<Client> {
	let client_hash = HashValue::from(client_hash);
	room.add_client_to_waiting_list(&client_hash, AccessGroups::from(group));
	room.client_connect(&client_hash).unwrap()
}

pub fn setup_listener(room: &mut Room) -> Rc<RefCell<Vec<String>>> {
	let results = Rc::new(RefCell::new(Vec::<String>::new()));
	let listener = Rc::new(RefCell::new(TestListener { results: results.clone() }));
	room.listener.add_listener(listener);
	results
}


struct TestListener {
	pub results: Rc<RefCell<Vec<String>>>
}

impl RoomListener for TestListener {
	fn set_current_client(&mut self, client: Rc<Client>) {
	}
	
	fn unset_current_client(&mut self) {
	}
	
	fn on_object_created(&mut self, game_object: &GameObject, _clients: &Clients) {
		let rc = self.results.clone();
		rc.borrow_mut().push(format!("on_object_create {:?}", game_object.id.id));
	}
	
	fn on_object_delete(&mut self, game_object: &GameObject, _clients: &Clients) {
		let rc = self.results.clone();
		rc.borrow_mut().push(format!("on_object_delete {:?}", game_object.id.id));
	}
	
	fn on_client_connect(&mut self, client: &Client, _objects: &Objects) {
		let rc = self.results.clone();
		rc.borrow_mut().push(format!("on_client_connect {}", client.configuration.id));
	}
	
	fn on_client_disconnect(&mut self, client: &Client) {
		let rc = self.results.clone();
		rc.borrow_mut().push(format!("on_client_disconnect {}", client.configuration.id));
	}
	
	fn on_object_long_counter_change(&mut self, field_id: u16, game_object: &GameObject, _clients: &Clients) {
		let rc = self.results.clone();
		rc.borrow_mut().push(format!("on_object_long_counter_change {:?} {} {}", game_object.id.id, field_id, game_object.get_long_counter(field_id)));
	}
	
	fn on_object_float_counter_change(&mut self, field_id: u16, game_object: &GameObject, _clients: &Clients) {
		let rc = self.results.clone();
		rc.borrow_mut().push(format!("on_object_float_counter_change {:?} {} {}", game_object.id.id, field_id, game_object.get_float_counter(field_id)));
	}
	
	fn on_object_event_fired(&mut self, field_id: u16, event_data: &[u8], game_object: &GameObject, _clients: &Clients) {
		let rc = self.results.clone();
		rc.borrow_mut().push(format!("on_object_event_fired {:?} {} {:?}", game_object.id.id, field_id, event_data));
	}
	
	fn on_object_struct_updated(&mut self, field_id: u16, game_object: &GameObject, _clients: &Clients) {
		let rc = self.results.clone();
		rc.borrow_mut().push(format!("on_object_struct_updated {:?} {} {:?}", game_object.id.id, field_id, game_object.get_struct(field_id).unwrap()));
	}
	
	fn on_object_long_counter_set(&mut self, field_id: u16, game_object: &GameObject, clients: &Clients) {
		let rc = self.results.clone();
		rc.borrow_mut().push(format!(
			"on_object_long_counter_set {:?} {} {:?}",
			game_object.id.id,
			field_id,
			game_object.get_long_counter(field_id)));
	}
	
	fn on_object_float_counter_set(&mut self, field_id: u16, game_object: &GameObject, clients: &Clients) {
		let rc = self.results.clone();
		rc.borrow_mut().push(format!(
			"on_object_float_counter_set {:?} {} {:?}",
			game_object.id.id,
			field_id,
			game_object.get_float_counter(field_id)));
	}
}