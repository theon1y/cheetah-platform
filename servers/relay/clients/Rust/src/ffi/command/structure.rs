use cheetah_relay_common::commands::command::C2SCommand;
use cheetah_relay_common::commands::command::meta::s2c::S2CMetaCommandInformation;
use cheetah_relay_common::commands::command::structure::StructureCommand;
use cheetah_relay_common::constants::FieldID;

use crate::ffi::{BufferFFI, execute_with_client, GameObjectIdFFI};
use crate::ffi::command::send_command;

#[no_mangle]
#[allow(unused_must_use)]
pub extern fn set_structure_listener(listener: extern fn(&S2CMetaCommandInformation, &GameObjectIdFFI, FieldID, &BufferFFI)) -> bool {
	execute_with_client(|client| {
		client.register_structure_listener(listener);
	}).is_ok()
}

#[no_mangle]
pub extern "C" fn set_structure(object_id: &GameObjectIdFFI, field_id: FieldID, structure: &BufferFFI) -> bool {
	send_command(C2SCommand::Structure(StructureCommand { object_id: From::from(object_id), field_id, structure: From::from(structure) }))
}