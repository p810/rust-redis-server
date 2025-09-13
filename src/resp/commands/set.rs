use std::time::Duration;

use crate::resp::commands::{RespCommandConstructor, RespCommandError};
use crate::resp::RespElement;
use crate::resp::types::RespArray;

#[derive(Debug)]
pub struct RespSetCommand {
    pub key: String,
    pub value: Box<[u8]>,
    pub ttl: Option<Duration>,
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
                match String::from_utf8(b.value.to_vec()) {
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
            RespElement::SimpleString(s) => s.value.as_bytes(),
            RespElement::BulkString(b) => &b.value,
            _ => return Err(RespCommandError::InvalidArgument)
        };

        let value: Box<[u8]> = Box::from(value);

        let ttl = match input.elements.get(3..=4) {
            Some([option, ttl_element]) => {
                let option_name = match option {
                    RespElement::SimpleString(s) => s.value.as_str(),
                    RespElement::BulkString(b) => {
                        match str::from_utf8(&b.value) {
                            Ok(as_string) => as_string,
                            Err(_) => return Err(RespCommandError::ParsingError),
                        }
                    },
                    _ => return Err(RespCommandError::ParsingError),
                };

                match option_name {
                    "EX" => get_ttl_in_secs(ttl_element)?,
                    _ => return Err(RespCommandError::InvalidArgument),
                }
            }
            _ => None,
        };

        Ok(RespSetCommand { key, value, ttl })
    }
}

fn get_ttl_in_secs(element: &RespElement) -> Result<Option<Duration>, RespCommandError> {
    let ttl_in_secs = match element {
        RespElement::Integer(i) if i.value > 0 => {
            if i.value > i32::MAX as isize {
                return Err(RespCommandError::InvalidArgument);
            }

            Some(Duration::from_secs(i.value as u64))
        }
        _ => return Err(RespCommandError::InvalidArgument),
    };

    Ok(ttl_in_secs)
}
