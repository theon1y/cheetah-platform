use cheetah_matches_relay_common::commands::c2s::C2SCommand;
use cheetah_matches_relay_common::commands::types::load::CreateGameObjectCommand;
use cheetah_matches_relay_common::commands::types::long::SetLongCommand;
use cheetah_matches_relay_common::commands::types::structure::SetStructureCommand;
use cheetah_matches_relay_common::protocol::codec::cipher::Cipher;
use cheetah_matches_relay_common::protocol::frame::applications::{BothDirectionCommand, CommandWithChannel};
use cheetah_matches_relay_common::protocol::frame::channel::Channel;
use cheetah_matches_relay_common::protocol::frame::Frame;

#[test]
// msgpack - raw(12), compressed(5), chiper(30)
// self - raw(7), compressed(4), chiper(25)
pub fn empty_frame() {
	let frame = Frame::new(100500);
	let mut buffer = [0; 2048];
	let private_key = [0; 32];
	let _size = frame.encode(&mut Cipher::new(&private_key), &mut buffer);
	//println!("{}", size);
	// 30 байт
}

#[test]
// msgpack - raw(61), compressed(40), chiper(66)
// self - raw(21), compressed(18), chiper(39)
pub fn create_object_frame() {
	let mut frame = Frame::new(100500);
	frame.reliable.push_back(CommandWithChannel {
		channel: Channel::ReliableUnordered,
		command: BothDirectionCommand::C2S(C2SCommand::Create(CreateGameObjectCommand {
			object_id: Default::default(),
			template: 0,
			access_groups: Default::default(),
		})),
	});
	frame.reliable.push_back(CommandWithChannel {
		channel: Channel::ReliableUnordered,
		command: BothDirectionCommand::C2S(C2SCommand::SetStructure(SetStructureCommand {
			object_id: Default::default(),
			field_id: 30,
			structure: Default::default(),
		})),
	});
	frame.reliable.push_back(CommandWithChannel {
		channel: Channel::ReliableUnordered,
		command: BothDirectionCommand::C2S(C2SCommand::SetLong(SetLongCommand {
			object_id: Default::default(),
			field_id: 55,
			value: 100,
		})),
	});

	let mut buffer = [0; 2048];
	let private_key = [0; 32];
	let _size = frame.encode(&mut Cipher::new(&private_key), &mut buffer);
	//println!("{}", size);
}