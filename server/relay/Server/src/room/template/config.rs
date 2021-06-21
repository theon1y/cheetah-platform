use std::collections::HashMap;

use fnv::FnvBuildHasher;

use cheetah_relay_common::constants::{FieldId, GameObjectTemplateId};
use cheetah_relay_common::room::access::AccessGroups;
use cheetah_relay_common::room::object::GameObjectId;
use cheetah_relay_common::room::UserPrivateKey;

use crate::room::types::FieldType;

///
/// Шаблон для создания комнаты
///
#[derive(Debug, Default, Clone)]
pub struct RoomTemplate {
	pub uid: String,
	pub objects: Vec<GameObjectTemplate>,
	pub permissions: Permissions,
}

#[derive(Debug, Default, Clone)]
pub struct UserTemplate {
	pub private_key: UserPrivateKey,
	pub access_groups: AccessGroups,
	pub objects: Vec<GameObjectTemplate>,
}

#[derive(Debug, Default, Clone)]
pub struct GameObjectTemplate {
	pub id: u32,
	pub template: GameObjectTemplateId,
	pub access_groups: AccessGroups,
	pub fields: GameObjectFieldsTemplate,
}

#[derive(Debug, Default, Clone)]
pub struct GameObjectFieldsTemplate {
	pub longs: HashMap<FieldId, i64, FnvBuildHasher>,
	pub floats: HashMap<FieldId, f64, FnvBuildHasher>,
	pub structures: HashMap<FieldId, rmpv::Value, FnvBuildHasher>,
}

#[derive(Debug, Default, Clone)]
pub struct Permissions {
	pub templates: Vec<TemplatePermission>,
}

#[derive(Debug, Default, Clone)]
pub struct TemplatePermission {
	pub template: GameObjectTemplateId,
	pub groups: Vec<PermissionGroup>,
	pub fields: Vec<PermissionField>,
}

#[derive(Debug, Copy, Clone)]
pub struct PermissionGroup {
	pub group: AccessGroups,
	pub permission: Permission,
}

#[derive(Debug, Clone)]
pub struct PermissionField {
	pub field_id: FieldId,
	pub field_type: FieldType,
	pub groups: Vec<PermissionGroup>,
}

#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
pub enum Permission {
	Deny,
	Ro,
	Rw,
}

#[derive(Debug)]
pub enum UserTemplateError {
	UserObjectHasWrongId(UserPrivateKey, u32),
}

impl UserTemplate {
	pub fn validate(self) -> Result<UserTemplate, UserTemplateError> {
		for object in &self.objects {
			if object.id >= GameObjectId::CLIENT_OBJECT_ID_OFFSET {
				return Result::Err(UserTemplateError::UserObjectHasWrongId(self.private_key, object.id));
			}
		}
		Result::Ok(self)
	}
}

#[cfg(test)]
mod tests {
	use crate::room::template::config::{
		GameObjectTemplate, Permission, PermissionField, PermissionGroup, Permissions, TemplatePermission, UserTemplate, UserTemplateError,
	};
	use crate::room::types::FieldType;
	use cheetah_relay_common::constants::{FieldId, GameObjectTemplateId};
	use cheetah_relay_common::room::access::AccessGroups;
	use cheetah_relay_common::room::object::GameObjectId;

	impl UserTemplate {
		pub fn stub(access_group: AccessGroups) -> Self {
			return UserTemplate {
				private_key: [5; 32],
				access_groups: access_group,
				objects: Default::default(),
			};
		}

		pub fn configure_object(&mut self, id: u32, template: GameObjectTemplateId, access_groups: AccessGroups) -> &mut GameObjectTemplate {
			let objects = &mut self.objects;
			objects.push(GameObjectTemplate {
				id,
				template,
				access_groups,
				fields: Default::default(),
			});
			let len = objects.len();
			let option = objects.get_mut(len - 1);
			option.unwrap()
		}
	}

	impl Permissions {
		pub fn set_permission(
			&mut self,
			template: GameObjectTemplateId,
			field_id: &FieldId,
			field_type: FieldType,
			access_group: &AccessGroups,
			permission: Permission,
		) {
			let template_permission = match self.templates.iter_mut().find(|t| t.template == template) {
				None => {
					let template_permission = TemplatePermission {
						template,
						groups: vec![],
						fields: vec![],
					};
					self.templates.push(template_permission);
					self.templates.iter_mut().find(|t| t.template == template).unwrap()
				}
				Some(template) => template,
			};

			let permission_field = match template_permission.fields.iter_mut().find(|f| f.field_id == *field_id) {
				None => {
					let permission_field = PermissionField {
						field_id: field_id.clone(),
						field_type,
						groups: vec![],
					};
					template_permission.fields.push(permission_field);
					template_permission.fields.iter_mut().find(|f| f.field_id == *field_id).unwrap()
				}
				Some(permission_field) => permission_field,
			};

			permission_field.groups.push(PermissionGroup {
				group: access_group.clone(),
				permission,
			});
		}
	}

	#[test]
	fn should_validate_fail_when_user_object_has_wrong_id() {
		let template = UserTemplate {
			private_key: [5; 32],
			access_groups: AccessGroups(0b1111),
			objects: vec![GameObjectTemplate {
				id: GameObjectId::CLIENT_OBJECT_ID_OFFSET + 1,
				template: 0b100,
				access_groups: AccessGroups(0b1111),
				fields: Default::default(),
			}],
		};

		assert!(matches!(template.validate(), Result::Err(UserTemplateError::UserObjectHasWrongId(_, _))))
	}
}
