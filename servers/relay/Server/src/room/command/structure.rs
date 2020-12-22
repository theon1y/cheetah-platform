use cheetah_relay_common::commands::command::structure::StructureCommand;
use cheetah_relay_common::commands::command::{HeaplessBuffer, S2CCommand};
use cheetah_relay_common::room::UserId;

use crate::room::command::ServerCommandExecutor;
use crate::room::object::{FieldIdAndType, GameObject, S2CommandWithFieldInfo};
use crate::room::template::config::Permission;
use crate::room::types::FieldType;
use crate::room::Room;

impl ServerCommandExecutor for StructureCommand {
	fn execute(self, room: &mut Room, user_id: &UserId) {
		let field_id = self.field_id;
		let object_id = self.object_id.clone();
		let action = |object: &mut GameObject| {
			object.structures.insert(self.field_id, self.structure.to_vec());
			Option::Some(S2CCommand::SetStruct(self))
		};
		room.do_action(&object_id, &field_id, FieldType::Structure, user_id, Permission::Rw, action);
	}
}

impl GameObject {
	pub fn structures_to_commands(&self, commands: &mut Vec<S2CommandWithFieldInfo>) {
		self.structures.iter().for_each(|(field_id, v)| {
			let structure = HeaplessBuffer::from_slice(&v.as_slice()).unwrap();
			commands.push(S2CommandWithFieldInfo {
				field: Option::Some(FieldIdAndType {
					field_id: field_id.clone(),
					field_type: FieldType::Structure,
				}),
				command: S2CCommand::SetStruct(StructureCommand {
					object_id: self.id.clone(),
					field_id: field_id.clone(),
					structure,
				}),
			});
		})
	}
}

#[cfg(test)]
mod tests {
	use cheetah_relay_common::commands::command::structure::StructureCommand;
	use cheetah_relay_common::commands::command::S2CCommand;
	use cheetah_relay_common::room::access::AccessGroups;

	use crate::room::command::ServerCommandExecutor;
	use crate::room::template::config::RoomTemplate;
	use crate::room::tests::from_vec;
	use crate::room::Room;

	#[test]
	pub fn should_set_structure() {
		let mut template = RoomTemplate::default();
		let access_groups = AccessGroups(10);
		let user = 1;
		template.configure_user(user, access_groups);
		let mut room = Room::from_template(template);
		let object_id = room.create_object(&user, access_groups).id.clone();

		room.out_commands.clear();
		let command = StructureCommand {
			object_id: object_id.clone(),
			field_id: 100,
			structure: from_vec(vec![1, 2, 3, 4, 5]),
		};

		command.clone().execute(&mut room, &user);
		let object = room.get_object_mut(&object_id).unwrap();

		assert_eq!(*object.structures.get(&100).unwrap(), command.structure.to_vec());
		assert!(matches!(room.out_commands.pop_back(), Some((.., S2CCommand::SetStruct(c))) if c==command));
	}
}
