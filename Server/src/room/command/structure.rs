use cheetah_relay_common::commands::command::S2CCommandUnion;
use cheetah_relay_common::commands::command::structure::StructureCommand;
use cheetah_relay_common::room::UserPublicKey;


use crate::room::Room;
use crate::room::command::ServerCommandExecutor;

impl ServerCommandExecutor for StructureCommand {
	fn execute(self, room: &mut dyn Room, _: &UserPublicKey) {
		if let Some(object) = room.get_object(&self.object_id) {
			object.fields.structures.insert(self.field_id, self.structure.clone());
			let groups = object.access_groups.clone();
			room.send_to_group(groups, S2CCommandUnion::SetStruct(self))
		}
	}
}


#[cfg(test)]
mod tests {
	use cheetah_relay_common::commands::command::S2CCommandUnion;
	use cheetah_relay_common::commands::command::structure::StructureCommand;
	use cheetah_relay_common::room::object::GameObjectId;
	use cheetah_relay_common::room::owner::ClientOwner;
	
	use crate::room::command::ServerCommandExecutor;
	use crate::room::Room;
	use crate::room::tests::RoomStub;
	
	#[test]
	pub fn should_set_structure() {
		let mut room = RoomStub::new();
		let object_id = room.create_object(&0).id.clone();
		let command = StructureCommand {
			object_id: object_id.clone(),
			field_id: 100,
			structure: vec![1, 2, 3, 4, 5],
		};
		
		command.clone().execute(&mut room, &32);
		let object = room.get_object(&object_id).unwrap();
		
		assert_eq!(*object.fields.structures.get(&100).unwrap(), command.structure);
		assert!(matches!(room.out_commands.pop_back(), Some((.., S2CCommandUnion::SetStruct(c))) if c==command));
	}
	
	#[test]
	pub fn should_not_panic_when_missing_object() {
		let mut room = RoomStub::new();
		let command = StructureCommand {
			object_id: GameObjectId::new(10, ClientOwner::Root),
			field_id: 100,
			structure: vec![1, 2, 3, 4, 5],
		};
		command.execute(&mut room, &32);
	}
}