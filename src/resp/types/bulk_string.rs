use crate::resp::RESP_DELIMITER;

use crate::resp::parser::{
    RespSerialize,
    RespElementConstructor,
    RespParseError,
    read_until_crlf,
    get_length_of_current_element,
};

#[derive(Debug)]
pub struct RespBulkString {
    pub length: usize,
    pub value: Box<[u8]>,
}

impl RespBulkString {
    pub fn new(value: &[u8]) -> RespBulkString {
        RespBulkString {
            value: value.into(),
            length: value.len(),
        }
    }
}

impl RespSerialize for RespBulkString {
    fn to_bytes(&self) -> Vec<u8> {
        let mut acc = vec![b'$'];

        acc.extend(self.length.to_string().as_bytes());
        acc.extend_from_slice(RESP_DELIMITER);

        acc.extend_from_slice(self.value.iter().as_slice());
        acc.extend_from_slice(RESP_DELIMITER);

        acc
    }
}

impl RespElementConstructor for RespBulkString {
    fn from_byte_slice(input: &[u8]) -> Result<(Self, &[u8]), RespParseError> {
        if input[0] != b'$' {
            return Err(RespParseError::UnknownTypePrefix);
        }

        let packet = &input[1..];

        let (length, remaining_bytes) = get_length_of_current_element(packet)?;
        
        let buffer: Box<[u8]> = Box::from(&remaining_bytes[0..length]);

        let (remainder_of_line, next_line) = read_until_crlf(&remaining_bytes[length..]).unwrap();

        if remainder_of_line.len() > 0 {
            return Err(RespParseError::InvalidElement);
        }

        Ok((RespBulkString { length, value: buffer }, next_line))
    }
}