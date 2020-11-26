use serde::{Deserialize, Serialize};

use crate::commands::command::event::EventCommand;
use crate::commands::command::float::{IncrementFloat64C2SCommand, SetFloat64Command};
use crate::commands::command::load::CreateGameObjectCommand;
use crate::commands::command::long::{IncrementLongC2SCommand, SetLongCommand};
use crate::commands::command::meta::c2s::C2SMetaCommandInformation;
use crate::commands::command::meta::s2c::S2CMetaCommandInformation;
use crate::commands::command::structure::StructureCommand;
use crate::commands::command::unload::DeleteGameObjectCommand;
use crate::room::object::GameObjectId;

pub mod event;
pub mod unload;
pub mod float;
pub mod long;
pub mod structure;
pub mod load;
pub mod meta;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum C2SCommand {
	Create(CreateGameObjectCommand),
	SetLongValue(SetLongCommand),
	IncrementLongValue(IncrementLongC2SCommand),
	SetFloatValue(SetFloat64Command),
	IncrementFloatCounter(IncrementFloat64C2SCommand),
	Structure(StructureCommand),
	Event(EventCommand),
	Delete(DeleteGameObjectCommand),
	Test(String),
	///
	/// Загрузить все объекты комнаты
	///
	AttachToRoom,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum S2CCommand {
	Create(CreateGameObjectCommand),
	SetLong(SetLongCommand),
	SetFloat64(SetFloat64Command),
	SetStruct(StructureCommand),
	Event(EventCommand),
	Delete(DeleteGameObjectCommand),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct S2CCommandWithMeta {
	pub meta: S2CMetaCommandInformation,
	pub command: S2CCommand,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct C2SCommandWithMeta {
	pub meta: C2SMetaCommandInformation,
	pub command: C2SCommand,
}


///
/// Метод получения идентификатора объекта, для команд выполняемых от имени объекта
///
pub trait GameObjectCommand {
	fn get_object_id(&self) -> &GameObjectId;
}