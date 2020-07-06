use cheetah_relay_client::client::command::C2SCommandUnion;
use cheetah_relay_client::client::ffi::{C2SCommandFFIType, Client2ServerFFIConverter, Command, ObjectIdType, S2CCommandFFIType, Server2ClientFFIConverter};
use cheetah_relay_client::client::ffi::bytes::Bytes;
use cheetah_relay_common::network::command::event::EventCommand;
use cheetah_relay_common::room::object::ClientGameObjectId;
use cheetah_relay_common::room::owner::ClientOwner;

#[test]
fn should_to_ffi() {
	let object_id = ClientGameObjectId::new(100, ClientOwner::Root);
	let command = EventCommand {
		object_id: object_id.clone(),
		field_id: 10,
		event: vec![1, 2, 3, 4, 5],
	};
	
	let mut ffi = Command::default();
	command.to_ffi(&mut ffi);
	
	assert_eq!(S2CCommandFFIType::Event, ffi.command_type_s2c);
	assert_eq!(object_id, ffi.object_id.to_common_game_object_id());
	assert_eq!(vec![1 as u8, 2, 3, 4, 5].as_slice(), ffi.event.as_slice())
}

#[test]
fn should_from_ffi() {
	let object_id = ClientGameObjectId::new(100, ClientOwner::Root);
	
	let mut ffi = Command::default();
	ffi.command_type_c2s = C2SCommandFFIType::Event;
	ffi.object_id.set_from(&object_id);
	ffi.field_id = 10;
	ffi.event = Bytes::from(vec![1, 2, 3]);
	let command = EventCommand::from_ffi(&ffi);
	
	assert!(matches!(&command,C2SCommandUnion::Event(ref event) if event.object_id == object_id));
	assert!(matches!(&command,C2SCommandUnion::Event(ref event) if event.field_id == 10));
	assert!(matches!(&command,C2SCommandUnion::Event(ref event) if event.event.as_slice() == vec![1,2,3].as_slice()));
}