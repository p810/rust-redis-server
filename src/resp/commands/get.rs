use crate::resp::commands::{RespCommandConstructor, RespCommandError};
use crate::resp::RespElement;
use crate::resp::types::RespArray;

#[derive(Debug)]
pub struct RespGetCommand {
    pub key: String,
}

impl RespCommandConstructor for RespGetCommand {
    fn from_array(input: &RespArray) -> Result<RespGetCommand, RespCommandError> {
        let Some(key_element) = input.elements.get(1) else {
            return Err(RespCommandError::InvalidArgument);
        };

        let key = match key_element {
            RespElement::SimpleString(s) => s.value.clone(),
            RespElement::BulkString(b) => {
                let Ok(as_string) = String::from_utf8(b.value.clone()) else {
                    return Err(RespCommandError::InvalidArgument);
                };
                
                as_string
            },
            _ => return Err(RespCommandError::InvalidArgument),
        };

        Ok(RespGetCommand { key })
    }
}
