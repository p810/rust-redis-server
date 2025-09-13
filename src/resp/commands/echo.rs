use crate::resp::commands::{RespCommandConstructor, RespCommandError};
use crate::resp::RespElement;
use crate::resp::types::RespArray;

#[derive(Debug)]
pub struct RespEchoCommand {
    pub value: String,
}

impl RespCommandConstructor for RespEchoCommand {
    fn from_array(input: &RespArray) -> Result<RespEchoCommand, RespCommandError> {
        let Some(input_string) = input.elements.get(1) else {
            return Err(RespCommandError::ParsingError);
        };

        let value = match input_string {
            RespElement::SimpleString(s) => s.value.clone(),
            RespElement::BulkString(b) => {
                match String::from_utf8(b.value.to_vec()) {
                    Ok(as_string) => as_string,
                    Err(_) => return Err(RespCommandError::ParsingError),
                }
            }
            _ => return Err(RespCommandError::InvalidArgument),
        };

        Ok(RespEchoCommand { value })
    }
}
