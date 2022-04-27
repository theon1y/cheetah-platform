use std::cell::RefCell;
use std::rc::Rc;

use cheetah_matches_relay_common::commands::s2c::S2CCommand;
use cheetah_matches_relay_common::commands::types::long::{CompareAndSetLongCommand, SetLongCommand};
use cheetah_matches_relay_common::commands::FieldType;
use cheetah_matches_relay_common::constants::FieldId;
use cheetah_matches_relay_common::room::object::GameObjectId;
use cheetah_matches_relay_common::room::RoomMemberId;

use crate::room::command::{ServerCommandError, ServerCommandExecutor};
use crate::room::object::{Field, GameObject, S2CommandWithFieldInfo};
use crate::room::template::config::Permission;
use crate::room::Room;

impl ServerCommandExecutor for CompareAndSetLongCommand {
	fn execute(&self, room: &mut Room, user_id: RoomMemberId) -> Result<(), ServerCommandError> {
		let object_id = self.object_id.clone();
		let field_id = self.field_id;
		let is_field_changed = Rc::new(RefCell::new(false));
		let action = |object: &mut GameObject| {
			let allow = match object.get_long(&self.field_id) {
				None => true,
				Some(value) => *value == self.current,
			};
			if allow {
				*is_field_changed.borrow_mut() = true;
				object.set_long(self.field_id, self.new)?;
				if self.reset.is_some() {
					object.set_compare_and_set_owner(self.field_id, user_id)?;
				}
				Ok(Some(S2CCommand::SetLong(SetLongCommand {
					object_id: self.object_id.clone(),
					field_id: self.field_id,
					value: self.new,
				})))
			} else {
				Ok(None)
			}
		};

		room.send_command_from_action(
			&object_id,
			Field {
				id: field_id,
				field_type: FieldType::Long,
			},
			user_id,
			Permission::Rw,
			Option::None,
			action,
		)?;

		if is_field_changed.take() {
			match self.reset {
				None => {
					room.get_member_mut(&user_id)?
						.compare_and_sets_cleaners
						.remove(&(object_id, field_id));
				}
				Some(reset_value) => {
					room.get_member_mut(&user_id)?
						.compare_and_sets_cleaners
						.insert((object_id, field_id), reset_value)
						.map_err(|_| ServerCommandError::Error("Overflow compare and sets cleaners".to_string()))?;
				}
			}
		}

		Ok(())
	}
}

pub fn reset_all_compare_and_set(
	room: &mut Room,
	user_id: RoomMemberId,
	compare_and_sets_cleaners: &heapless::FnvIndexMap<(GameObjectId, FieldId), i64, 256>,
) -> Result<(), ServerCommandError> {
	for ((object_id, field), reset) in compare_and_sets_cleaners {
		match room.get_object(object_id) {
			Err(_) => {
				// нормальная ситуация для пользовательских объектов
			}
			Ok(object) => {
				if let Some(owner) = object.get_compare_and_set_owner(field) {
					if *owner == user_id {
						object.set_long(*field, *reset)?;
						let command = [S2CommandWithFieldInfo {
							field: Some(Field {
								id: *field,
								field_type: FieldType::Long,
							}),
							command: S2CCommand::SetLong(SetLongCommand {
								object_id: object_id.clone(),
								field_id: *field,
								value: *reset,
							}),
						}];
						let groups = object.access_groups;
						let template = object.template_id;
						room.send_to_members(groups, template, &command, |_| true)?
					}
				}
			}
		}
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use cheetah_matches_relay_common::commands::s2c::S2CCommand;
	use cheetah_matches_relay_common::commands::types::long::CompareAndSetLongCommand;
	use cheetah_matches_relay_common::commands::FieldType;
	use cheetah_matches_relay_common::constants::FieldId;
	use cheetah_matches_relay_common::room::access::AccessGroups;
	use cheetah_matches_relay_common::room::object::GameObjectId;
	use cheetah_matches_relay_common::room::owner::GameObjectOwner;
	use cheetah_matches_relay_common::room::RoomMemberId;

	use crate::room::command::ServerCommandExecutor;
	use crate::room::object::Field;
	use crate::room::template::config::{
		GameObjectTemplatePermission, GroupsPermissionRule, MemberTemplate, Permission, PermissionField, RoomTemplate,
	};
	use crate::room::Room;

	///
	/// Проверяем что при выполнении нескольких команд соблюдаются гарантии CompareAndSet
	///
	#[test]
	fn should_compare_and_set() {
		let (mut room, user1_id, _, object_id, field_id) = setup();
		let command1 = CompareAndSetLongCommand {
			object_id: object_id.clone(),
			field_id,
			current: 0,
			new: 100,
			reset: None,
		};
		command1.execute(&mut room, user1_id).unwrap();
		assert_eq!(
			*room.get_object(&object_id).unwrap().get_long(&command1.field_id).unwrap(),
			command1.new
		);

		let command2 = CompareAndSetLongCommand {
			object_id: object_id.clone(),
			field_id: command1.field_id,
			current: 0,
			new: 200,
			reset: None,
		};
		command2.execute(&mut room, user1_id).unwrap();
		assert_eq!(
			*room.get_object(&object_id).unwrap().get_long(&command1.field_id).unwrap(),
			command1.new
		);

		let command3 = CompareAndSetLongCommand {
			object_id: object_id.clone(),
			field_id: command1.field_id,
			current: command1.new,
			new: 300,
			reset: None,
		};
		command3.execute(&mut room, user1_id).unwrap();
		assert_eq!(
			*room.get_object(&object_id).unwrap().get_long(&command1.field_id).unwrap(),
			command3.new
		);
	}

	///
	/// Проверяем что команда отсылает изменения другим клиентам
	///
	#[test]
	fn should_send_command() {
		let (mut room, user1_id, _, object_id, field_id) = setup();
		let command = CompareAndSetLongCommand {
			object_id,
			field_id,
			current: 0,
			new: 100,
			reset: Some(555),
		};

		room.test_out_commands.clear();
		command.execute(&mut room, user1_id).unwrap();
		assert!(
			matches!(room.test_out_commands.pop_back(), Some((.., S2CCommand::SetLong(c))) if
			c.value==command.new)
		);
	}

	///
	/// Проверяем что при выходе пользователя будет установлено заданное значение
	///
	#[test]
	fn should_reset() {
		let (mut room, user1_id, _, object_id, field_id) = setup();
		let command = CompareAndSetLongCommand {
			object_id: object_id.clone(),
			field_id,
			current: 0,
			new: 100,
			reset: Some(555),
		};
		command.execute(&mut room, user1_id).unwrap();
		assert_eq!(
			*room.get_object(&object_id).unwrap().get_long(&command.field_id).unwrap(),
			command.new
		);

		room.disconnect_user(user1_id).unwrap();
		assert_eq!(
			*room.get_object(&object_id).unwrap().get_long(&command.field_id).unwrap(),
			command.reset.unwrap()
		);
	}

	///
	/// Проверяем что при выходе пользователя не будет сброшено значение, если была вторая
	/// команда CompareAndSet без установки reset
	///
	#[test]
	fn should_disable_reset() {
		let (mut room, user1_id, _, object_id, field_id) = setup();
		CompareAndSetLongCommand {
			object_id: object_id.clone(),
			field_id,
			current: 0,
			new: 100,
			reset: Some(555),
		}
		.execute(&mut room, user1_id)
		.unwrap();
		CompareAndSetLongCommand {
			object_id: object_id.clone(),
			field_id,
			current: 100,
			new: 200,
			reset: None,
		}
		.execute(&mut room, user1_id)
		.unwrap();

		assert_eq!(*room.get_object(&object_id).unwrap().get_long(&field_id).unwrap(), 200);
		room.disconnect_user(user1_id).unwrap();
		assert_eq!(*room.get_object(&object_id).unwrap().get_long(&field_id).unwrap(), 200);
	}

	///
	/// Проверяем,что если два пользователя друг за другом поменяли значения,
	/// то при разрыве соединения первого пользователя данные не будут заменены
	///
	#[test]
	fn should_correct_reset_when_with_two_members() {
		let (mut room, user1_id, user2_id, object_id, field_id) = setup();
		let command_1 = CompareAndSetLongCommand {
			object_id: object_id.clone(),
			field_id,
			current: 0,
			new: 100,
			reset: Some(555),
		};
		let command_2 = CompareAndSetLongCommand {
			object_id: object_id.clone(),
			field_id,
			current: 100,
			new: 200,
			reset: Some(1555),
		};
		command_1.execute(&mut room, user1_id).unwrap();
		command_2.execute(&mut room, user2_id).unwrap();

		room.disconnect_user(user1_id).unwrap();
		assert_eq!(
			*room.get_object(&object_id).unwrap().get_long(&command_1.field_id).unwrap(),
			command_2.new
		);
	}

	fn setup() -> (Room, RoomMemberId, RoomMemberId, GameObjectId, FieldId) {
		let access_group = AccessGroups(55);
		let mut template = RoomTemplate::default();
		let user_template_1 = MemberTemplate {
			private_key: Default::default(),
			groups: access_group,
			objects: Default::default(),
		};
		let user_template_2 = MemberTemplate {
			private_key: Default::default(),
			groups: access_group,
			objects: Default::default(),
		};

		let user_template_3 = MemberTemplate {
			private_key: Default::default(),
			groups: access_group,
			objects: Default::default(),
		};

		let object_template = 10;
		let object_field = 50;
		template.permissions.templates.push(GameObjectTemplatePermission {
			template: object_template,
			rules: vec![],
			fields: vec![PermissionField {
				field: Field {
					id: object_field,
					field_type: FieldType::Long,
				},
				rules: vec![GroupsPermissionRule {
					groups: access_group,
					permission: Permission::Rw,
				}],
			}],
		});
		let mut room = Room::from_template(template);
		let user1_id = room.register_member(user_template_1);
		let user2_id = room.register_member(user_template_2);
		let user3_id = room.register_member(user_template_3);
		let object = room.test_create_object_with_not_created_state(GameObjectOwner::Member(user3_id), access_group);
		object.created = true;
		object.template_id = object_template;

		let object_id = object.id.clone();
		(room, user1_id, user2_id, object_id, object_field)
	}
}