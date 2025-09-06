use crate::resp::RespElement;
use crate::resp::parser::{RespElementConstructor, RespParseError, get_length_of_current_element};

#[derive(Debug)]
pub struct RespArray {
    pub length: usize,
    pub elements: Vec<RespElement>,
}

impl RespElementConstructor for RespArray {
    fn from_byte_slice(slice: &[u8]) -> Result<(Self, &[u8]), RespParseError> {
        if slice[0] != b'*' {
            return Err(RespParseError::UnknownTypePrefix);
        }

        let packet = &slice[1..];

        let (length, mut remaining_bytes) = get_length_of_current_element(packet)?;

        let mut elements: Vec<RespElement> = Vec::with_capacity(length);

        for _ in 0 .. length {
            let (element, next_slice) = RespElement::from_byte_slice(remaining_bytes)?;

            elements.push(element);

            remaining_bytes = next_slice;
        };

        Ok((RespArray { length, elements }, remaining_bytes))
    }
}