use crate::resp::types::RespArray;
use crate::resp::{RespElement, RespDeserialize};

pub mod echo;
pub use echo::RespEchoCommand;

pub mod set;
pub use set::RespSetCommand;

pub mod get;
pub use get::RespGetCommand;

#[derive(Debug)]
pub enum RespCommand {
    Ping,
    Echo(RespEchoCommand),
    Set(RespSetCommand),
    Get(RespGetCommand),
}

#[derive(Debug)]
pub enum RespCommandError {
    UnknownCommand,
    InvalidArgument,
    ParsingError,
}

pub trait RespCommandConstructor {
    fn from_array(array: RespArray) -> Result<Self, RespCommandError>
    where
        Self: Sized;
}

impl RespCommandConstructor for RespCommand {
    fn from_array(input: RespArray) -> Result<RespCommand, RespCommandError> {
        let Some(first_element) = input.elements.first() else {
            return Err(RespCommandError::ParsingError);
        };

        let command = match first_element {
            RespElement::BulkString(b) => get_command_name(&b.value),
            _ => Err(RespCommandError::InvalidArgument),
        }?;

        let result = match command.to_lowercase().as_str() {
            "echo" => RespCommand::Echo(RespEchoCommand::from_array(input)?),
            "set" => RespCommand::Set(RespSetCommand::from_array(input)?),
            "get" => RespCommand::Get(RespGetCommand::from_array(input)?),
            _ => return Err(RespCommandError::UnknownCommand),
        };

        Ok(result)
    }
}

pub fn get_command_from_input(packet: &[u8]) -> Result<RespCommand, RespCommandError> {
    match RespElement::from_byte_slice(packet) {
        Ok((kind, _)) => {
            match kind {
                RespElement::Array(a) =>
                    RespCommand::from_array(a),
                RespElement::SimpleString(s) => {
                    if s.value.to_lowercase() == "ping" {
                        Ok(RespCommand::Ping)
                    } else {
                        Err(RespCommandError::UnknownCommand)
                    }
                },
                _ => Err(RespCommandError::ParsingError),
            }
        }
        Err(_) => Err(RespCommandError::ParsingError),
    }
}

fn get_command_name(bytes: &[u8]) -> Result<String, RespCommandError> {
    if !bytes.into_iter().all(| byte | byte.is_ascii_alphabetic()) {
        return Err(RespCommandError::ParsingError);
    }

    let Ok(command) = String::from_utf8(bytes.to_vec()) else {
        return Err(RespCommandError::ParsingError);
    };

    Ok(command)
}