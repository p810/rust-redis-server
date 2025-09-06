use crate::resp::commands::{RespCommandConstructor, RespCommandError};
use crate::resp::RespElement;
use crate::resp::types::RespArray;

#[derive(Debug)]
pub struct RespSetCommand {
    pub key: String,
    pub value: String,
}

impl RespCommandConstructor for RespSetCommand {
    fn from_array(input: &RespArray) -> Result<RespSetCommand, RespCommandError> {
        if input.length < 3 {
            return Err(RespCommandError::InvalidArgument);
        }

        let Some(key_element) = input.elements.get(1) else {
            return Err(RespCommandError::InvalidArgument);
        };

        let key = match key_element {
            RespElement::SimpleString(s) => s.value.clone(),
            RespElement::BulkString(b) => {
                match String::from_utf8(b.value.clone()) {
                    Ok(as_string) => as_string,
                    Err(_) => return Err(RespCommandError::ParsingError)
                }
            }
            _ => return Err(RespCommandError::InvalidArgument),
        };

        let Some(value_element) = input.elements.get(2) else {
            return Err(RespCommandError::InvalidArgument);
        };

        let value = match value_element {
            RespElement::SimpleString(s) => s.value.clone(),
            RespElement::BulkString(b) => {
                match String::from_utf8(b.value.clone()) {
                    Ok(as_string) => as_string,
                    Err(_) => return Err(RespCommandError::ParsingError)
                }
            }
            _ => return Err(RespCommandError::InvalidArgument)
        };

        Ok(RespSetCommand { key, value })
    }
}
