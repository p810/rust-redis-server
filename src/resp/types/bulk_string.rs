use crate::resp::parser::{
    RespElementConstructor,
    RespParseError,
    read_until_crlf,
    get_length_of_current_element,
};

#[derive(Debug)]
pub struct RespBulkString {
    pub length: usize,
    pub value: Vec<u8>,
}

impl RespElementConstructor for RespBulkString {
    fn from_byte_slice(input: &[u8]) -> Result<(Self, &[u8]), RespParseError> {
        if input[0] != b'$' {
            return Err(RespParseError::UnknownTypePrefix);
        }

        let packet = &input[1..];

        let (length, remaining_bytes) = get_length_of_current_element(packet)?;
        
        let buffer: Vec<u8> = remaining_bytes[0..length].to_vec();

        let (remainder_of_line, next_line) = read_until_crlf(&remaining_bytes[length..]).unwrap();

        if remainder_of_line.len() > 0 {
            return Err(RespParseError::InvalidElement);
        }

        Ok((RespBulkString { length, value: buffer }, next_line))
    }
}