use std::collections::VecDeque;
use std::io::Cursor;

use thiserror::Error;

use crate::commands::c2s::{C2SCommand, C2SCommandDecodeError};
use crate::commands::s2c::{S2CCommand, S2CCommandDecodeError, S2CCommandWithCreator};
use crate::protocol::codec::channel::CommandChannelDecodeError;
use crate::protocol::codec::commands::context::{CommandContext, CommandContextError};
use crate::protocol::codec::commands::header::CommandHeader;
use crate::protocol::codec::variable_int::VariableIntReader;
use crate::protocol::frame::applications::{BothDirectionCommand, CommandWithChannel};
use crate::protocol::frame::channel::Channel;

///
/// Преобразование массива байт в список команд
///
pub fn decode_commands(
	from_client: bool,
	input: &mut Cursor<&[u8]>,
	out: &mut VecDeque<CommandWithChannel>,
) -> Result<(), CommandsDecoderError> {
	let length = input.read_variable_u64()?;
	let mut context = CommandContext::default();
	for _ in 0..length {
		let header = context.read_next(input)?;
		let command = decode_command(from_client, input, &header, &context)?;
		out.push_back(command);
	}
	Ok(())
}

fn decode_command(
	from_client: bool,
	input: &mut Cursor<&[u8]>,
	header: &CommandHeader,
	context: &CommandContext,
) -> Result<CommandWithChannel, CommandsDecoderError> {
	Ok(CommandWithChannel {
		channel: Channel::decode(&header.channel_type_id, context.get_channel_group_id(), input)?,
		command: match from_client {
			true => BothDirectionCommand::C2S(C2SCommand::decode(
				&header.command_type_id,
				context.get_object_id(),
				context.get_field_id(),
				input,
			)?),
			false => BothDirectionCommand::S2CWithCreator(S2CCommandWithCreator {
				creator: context.get_creator()?,
				command: S2CCommand::decode(
					&header.command_type_id,
					context.get_object_id(),
					context.get_field_id(),
					input,
				)?,
			}),
		},
	})
}

#[derive(Error, Debug)]
pub enum CommandsDecoderError {
	#[error("IO error {:?}", .source)]
	Io {
		#[from]
		source: std::io::Error,
	},

	#[error("ApplicationCommandChannel error {:?}", .source)]
	ApplicationCommandChannel {
		#[from]
		source: CommandChannelDecodeError,
	},

	#[error("C2SCommandDecodeError error {:?}", .source)]
	C2SCommandDecode {
		#[from]
		source: C2SCommandDecodeError,
	},

	#[error("CommandContextError error {:?}", .source)]
	CommandContext {
		#[from]
		source: CommandContextError,
	},
	#[error("S2CCommandDecodeError error {:?}", .source)]
	S2CCommandDecode {
		#[from]
		source: S2CCommandDecodeError,
	},
}