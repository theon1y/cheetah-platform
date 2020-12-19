#[cfg(test)]
use std::collections::VecDeque;

use cheetah_relay_common::commands::command::meta::c2s::C2SMetaCommandInformation;
use cheetah_relay_common::commands::command::meta::s2c::S2CMetaCommandInformation;
use cheetah_relay_common::commands::command::{S2CCommand, S2CCommandWithMeta};
use cheetah_relay_common::constants::FieldId;
use cheetah_relay_common::protocol::frame::applications::{ApplicationCommand, ApplicationCommandChannelType};
use cheetah_relay_common::room::access::AccessGroups;
use cheetah_relay_common::room::object::GameObjectId;
use cheetah_relay_common::room::owner::ObjectOwner;
use cheetah_relay_common::room::UserPublicKey;

use crate::room::object::GameObject;
use crate::room::template::config::Permission;
use crate::room::types::FieldType;
use crate::room::{Room, User};

impl Room {
	pub fn send_object_to_group(&mut self, object: &GameObject) {
		let mut commands = Vec::new();
		object.collect_create_commands(&mut commands);
		commands.into_iter().for_each(|c| {
			self.send_to_group(object.access_groups, c, |_| true);
		})
	}

	///
	/// Проверить права доступа, выполнить действие, результат выполнения отправить клиентам
	///
	/// - владелец объекта получает обновления если только данные доступны на запись другим клиентам
	/// - владелец объекта имеет полный доступ к полям объекта, информация о правах игнорируется
	///
	pub fn do_action<T>(
		&mut self,
		game_object_id: &GameObjectId,
		field_id: &FieldId,
		field_type: FieldType,
		command_owner_user: &UserPublicKey,
		permission: Permission,
		action: T,
	) where
		T: FnOnce(&mut GameObject) -> Option<S2CCommand>,
	{
		let room_id = self.id;

		let permission_manager = self.permission_manager.clone();

		let current_user_access_group = match self.users.get(command_owner_user) {
			None => {
				log::error!("[room({})] user({}) not found", self.id, command_owner_user);
				return;
			}
			Some(user) => user.template.access_groups.clone(),
		};

		if let Some(object) = self.get_object_mut(&game_object_id) {
			// проверяем группу доступа
			if !object.access_groups.contains_any(&current_user_access_group) {
				log::error!(
					"[room({})] user({}) group({:?}) don't allow access to object ({:?})",
					room_id,
					command_owner_user,
					current_user_access_group,
					object.access_groups
				);
				return;
			}

			let object_owner = if let ObjectOwner::User(owner) = object.id.owner {
				Option::Some(owner)
			} else {
				Option::None
			};

			let current_user_is_object_owner = object_owner == Option::Some(*command_owner_user);
			let allow = current_user_is_object_owner
				|| permission_manager
					.borrow_mut()
					.get_permission(object.template, *field_id, field_type, current_user_access_group)
					>= permission;

			if !allow {
				log::error!(
					"[room({:?})] user({:?}) has not permissions({:?}) for action with object({:?}), field({:?}), field_type({:?})",
					self.id,
					command_owner_user,
					permission,
					game_object_id,
					field_id,
					field_type
				);
				return;
			}

			let command = action(object);
			let groups = object.access_groups.clone();
			let template = object.template;

			if let Some(command) = command {
				self.send_to_group(groups, command, |user| {
					let mut permission_manager = permission_manager.borrow_mut();
					if object_owner == Option::Some(user.template.public_key) {
						permission_manager.has_write_access(template, *field_id, field_type)
					} else {
						permission_manager.get_permission(template, *field_id, field_type, user.template.access_groups) > Permission::Deny
					}
				});
			};
		};
	}

	pub fn send_to_group<T>(&mut self, access_groups: AccessGroups, command: S2CCommand, filter: T)
	where
		T: Fn(&User) -> bool,
	{
		#[cfg(test)]
		self.out_commands.push_front((access_groups, command.clone()));

		let channel_type = self
			.current_channel
			.as_ref()
			.unwrap_or(&ApplicationCommandChannelType::ReliableSequenceByGroup(0));

		let meta = match &self.current_user {
			None => S2CMetaCommandInformation::new(0, &C2SMetaCommandInformation { timestamp: 0 }),
			Some(user) => S2CMetaCommandInformation::new(
				user.clone(),
				self.current_meta.as_ref().unwrap_or(&C2SMetaCommandInformation { timestamp: 0 }),
			),
		};

		let application_command = ApplicationCommand::S2CCommandWithMeta(S2CCommandWithMeta {
			meta,
			command: command.clone(),
		});

		let room_id = self.id;
		let tracer = self.tracer.clone();

		#[cfg(test)]
		let test_commands_by_user = self.out_commands_by_users.clone();

		self.users
			.values_mut()
			.filter(|user| user.attached)
			.filter(|user| user.protocol.is_some())
			.filter(|user| user.template.access_groups.contains_any(&access_groups))
			.filter(|user| filter(user))
			.for_each(|user| {
				let protocol = user.protocol.as_mut().unwrap();
				tracer.on_s2c_command(room_id, user.template.public_key.clone(), &command);
				protocol
					.out_commands_collector
					.add_command(channel_type.clone(), application_command.clone());

				#[cfg(test)]
				{
					let user_public_key = user.template.public_key.clone();
					let mut commands = test_commands_by_user.borrow_mut();
					let user_commands = commands.entry(user_public_key).or_insert(VecDeque::new());
					user_commands.push_front(command.clone());
				}
			});
	}

	pub fn send_to_user(&mut self, user_public_key: &u32, commands: Vec<S2CCommand>) {
		#[cfg(test)]
		{
			let mut out_commands_by_user = self.out_commands_by_users.borrow_mut();
			let entry = out_commands_by_user.entry(user_public_key.clone());
			let user_commands = entry.or_insert(VecDeque::new());
			for command in &commands {
				user_commands.push_front(command.clone());
			}
		}

		match self.users.get_mut(user_public_key) {
			None => {
				log::error!("[room] send to unknown user {:?}", user_public_key)
			}
			Some(user) => {
				if let Some(ref mut protocol) = user.protocol {
					if user.attached {
						for command in commands {
							self.tracer.on_s2c_command(self.id, user.template.public_key, &command);
							let meta = self.current_meta.as_ref().unwrap();
							let channel = self.current_channel.as_ref().unwrap();
							let application_command = ApplicationCommand::S2CCommandWithMeta(S2CCommandWithMeta {
								meta: S2CMetaCommandInformation::new(user_public_key.clone(), meta),
								command,
							});
							protocol.out_commands_collector.add_command(channel.clone(), application_command.clone());
						}
					}
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use std::time::Instant;

	use cheetah_relay_common::commands::command::event::EventCommand;
	use cheetah_relay_common::commands::command::long::SetLongCommand;
	use cheetah_relay_common::commands::command::meta::c2s::C2SMetaCommandInformation;
	use cheetah_relay_common::commands::command::S2CCommand;
	use cheetah_relay_common::protocol::frame::applications::ApplicationCommandChannelType;
	use cheetah_relay_common::protocol::relay::RelayProtocol;
	use cheetah_relay_common::room::access::AccessGroups;

	use crate::room::template::config::{Permission, RoomTemplate};
	use crate::room::tests::create_template;
	use crate::room::types::FieldType;
	use crate::room::Room;

	#[test]
	fn should_send_command_to_other_user() {
		let (template, user_template) = create_template();
		let mut room = Room::from_template(template);
		room.current_user.replace(user_template.public_key + 1); // команда пришла от другого пользователя
		room.current_meta.replace(C2SMetaCommandInformation { timestamp: 0 });
		room.current_channel.replace(ApplicationCommandChannelType::ReliableSequenceByGroup(0));

		let user = room.get_user_mut(&user_template.public_key).unwrap();
		user.attached = true;
		user.protocol.replace(RelayProtocol::new(&Instant::now()));

		room.send_to_group(
			user_template.access_groups.clone(),
			S2CCommand::Event(EventCommand {
				object_id: Default::default(),
				field_id: 0,
				event: Default::default(),
			}),
			|_| true,
		);

		let user = room.get_user(&user_template.public_key).unwrap();
		let protocol = user.protocol.as_ref().unwrap();
		assert_eq!(protocol.out_commands_collector.commands.reliable.len(), 1);
	}

	#[test]
	fn should_do_action_check_execute_only_with_enough_permission() {
		let mut template = RoomTemplate::default();
		let access_groups = AccessGroups(55);
		let user_1 = template.create_user(1, access_groups);
		let user_2 = template.create_user(2, access_groups);

		let field_id_1 = 10;
		let field_id_2 = 11;
		template
			.permissions
			.set_permission(0, &field_id_2, FieldType::Long, &access_groups, Permission::Rw);

		let mut room = Room::from_template(template);
		let object_id = room.create_object_with_access_groups(&user_1, access_groups).id.clone();

		// владельцу разрешены любые операции
		let mut executed = false;
		room.do_action(&object_id, &field_id_1, FieldType::Long, &user_1, Permission::Rw, |_| {
			executed = true;
			None
		});
		assert!(executed);

		// RO - по-умолчанию для всех полей
		let mut executed = false;
		room.do_action(&object_id, &field_id_1, FieldType::Long, &user_2, Permission::Ro, |_| {
			executed = true;
			None
		});
		assert!(executed);

		// RW - по-умолчанию запрещен
		let mut executed = false;
		room.do_action(&object_id, &field_id_1, FieldType::Long, &user_2, Permission::Rw, |_| {
			executed = true;
			None
		});
		assert!(!executed);

		// RW - разрешен для второго поля
		let mut executed = false;
		room.do_action(&object_id, &field_id_2, FieldType::Long, &user_2, Permission::Rw, |_| {
			executed = true;
			None
		});
		assert!(executed);
	}

	///
	/// Посылка обратной команды зависит от того изменяют ли поле один клиент или множество
	///
	#[test]
	fn should_do_action_check_send_callback() {
		let mut template = RoomTemplate::default();
		let access_groups = AccessGroups(55);
		let field_id_1 = 10;
		let field_id_2 = 20;
		let field_type = FieldType::Long;
		template
			.permissions
			.set_permission(0, &field_id_2, field_type, &access_groups, Permission::Rw);

		let user = template.create_user(1, access_groups);
		let mut room = Room::from_template(template);
		let object = room.create_object(&user);
		object.access_groups = access_groups.clone();
		let object_id = object.id.clone();
		room.mark_as_connected(&user);

		// изменяем поле, которое никто кроме нас не может изменять
		let mut executed = false;
		room.do_action(&object_id, &field_id_1, field_type, &user, Permission::Rw, |_| {
			executed = true;
			Option::Some(S2CCommand::SetLong(SetLongCommand {
				object_id: object_id.clone(),
				field_id: field_id_1,
				value: 0,
			}))
		});
		assert!(executed);
		assert_eq!(room.out_commands_by_users.borrow().get(&user).is_none(), true);

		// изменяем поле, которое могут изменять другие пользователи
		let mut executed = false;
		room.do_action(&object_id, &field_id_2, field_type, &user, Permission::Rw, |_| {
			executed = true;
			Option::Some(S2CCommand::SetLong(SetLongCommand {
				object_id: object_id.clone(),
				field_id: field_id_2,
				value: 0,
			}))
		});
		assert!(executed);
		assert!(matches!(
			room.out_commands_by_users.borrow().get(&user).unwrap().get(0),
			Option::Some(S2CCommand::SetLong(_))
		));
	}

	///
	/// Не посылать обновление клиенту если это запрещено правами
	///
	#[test]
	fn should_do_action_check_send_update() {
		let mut template = RoomTemplate::default();
		let access_groups_a = AccessGroups(0b111);
		let access_groups_b = AccessGroups(0b100);
		let user_1 = template.create_user(1, access_groups_a);
		let user_2 = template.create_user(2, access_groups_b);

		let field_id_1 = 10;
		let field_id_2 = 11;
		let field_type = FieldType::Long;
		template
			.permissions
			.set_permission(0, &field_id_2, FieldType::Long, &access_groups_b, Permission::Deny);

		let mut room = Room::from_template(template);

		room.mark_as_connected(&user_1);
		room.mark_as_connected(&user_2);

		let object_id = room.create_object_with_access_groups(&user_1, access_groups_a).id.clone();

		// изменяем поле, доступное другому пользователю - он должен получить обновление
		let mut executed = false;
		room.do_action(&object_id, &field_id_1, field_type, &user_1, Permission::Rw, |_| {
			executed = true;
			Option::Some(S2CCommand::SetLong(SetLongCommand {
				object_id: object_id.clone(),
				field_id: field_id_2,
				value: 0,
			}))
		});
		assert!(executed);
		assert!(matches!(
			room.out_commands_by_users.borrow().get(&user_2).unwrap().get(0),
			Option::Some(S2CCommand::SetLong(_))
		));

		// изменяем поле, не доступное другому пользователю - он не должен получить обновление
		let mut executed = false;
		room.do_action(&object_id, &field_id_2, field_type, &user_1, Permission::Rw, |_| {
			executed = true;
			Option::Some(S2CCommand::SetLong(SetLongCommand {
				object_id: object_id.clone(),
				field_id: field_id_2,
				value: 0,
			}))
		});
		assert!(executed);
		assert!(matches!(room.out_commands_by_users.borrow().get(&user_2).unwrap().get(1), Option::None));
	}

	///
	/// Действие не должно выполнится если пользователь не входит в группу объекта
	///
	#[test]
	fn should_do_action_with_object_from_correct_group() {
		let mut template = RoomTemplate::default();
		let access_groups_a = AccessGroups(0b01);
		let access_groups_b = AccessGroups(0b10);
		let user_1 = template.create_user(1, access_groups_a);
		let user_2 = template.create_user(2, access_groups_b);

		let mut room = Room::from_template(template);
		let object_id = room.create_object_with_access_groups(&user_1, access_groups_a).id.clone();

		let mut executed = false;
		room.do_action(&object_id, &0, FieldType::Long, &user_2, Permission::Ro, |_| {
			executed = true;
			None
		});
		assert!(!executed);
	}
}
