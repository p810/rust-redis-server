use crate::resp::{RespElement, RESP_DELIMITER};
use crate::resp::parser::{
    RespSerialize,
    RespElementConstructor,
    RespParseError,
    get_length_of_current_element,
};

#[derive(Debug)]
pub struct RespArray {
    pub length: usize,
    pub elements: Vec<RespElement>,
}

impl RespArray {
    pub fn new(elements: Vec<RespElement>) -> RespArray {
        let length = elements.len();

        RespArray {
            length,
            elements,
        }
    }
}

impl RespSerialize for RespArray {
    fn to_bytes(&self) -> Vec<u8> {
        let mut acc = vec![b'*'];

        acc.extend(self.length.to_string().as_bytes());
        acc.extend_from_slice(RESP_DELIMITER);

        let elements = self.elements.iter().flat_map(| e | e.to_bytes());

        acc.extend(elements);

        acc
    }
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