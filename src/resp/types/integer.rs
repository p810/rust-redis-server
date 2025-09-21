use crate::resp::parser::{
    RespSerialize,
    RespDeserialize,
    RespParseError,
    read_until_crlf,
};

#[derive(Debug)]
pub struct RespInteger {
    pub value: isize,
}

impl RespSerialize for RespInteger {
    fn to_bytes(&self) -> Vec<u8> {
        let mut acc = vec![b':'];

        acc.extend_from_slice(self.value.to_string().as_bytes());

        acc
    }
}

impl RespDeserialize for RespInteger {
    fn from_byte_slice(input: &[u8]) -> Result<(Self, &[u8]), RespParseError> {
        if input[0] != b':' {
            return Err(RespParseError::UnknownTypePrefix);
        }

        let (start, multiplier): (usize, isize) = match input.get(1) {
            Some(char) => {
                match char {
                    b'+' => (2, 1),
                    b'-' => (2, -1),
                    _ => (1, 1),
                }
            }
            None => return Err(RespParseError::UnexpectedEof),
        };

        match read_until_crlf(&input[start..]) {
            Some((data, remaining_bytes)) => {
                let Ok(as_string) = String::from_utf8(data.to_vec()) else {
                    return Err(RespParseError::InvalidElement);
                };

                let Ok(as_int) = as_string.parse::<usize>() else {
                    return Err(RespParseError::InvalidElement);
                };

                let value = (as_int as isize) * multiplier;

                Ok((RespInteger { value }, remaining_bytes))
            }
            None => Err(RespParseError::InvalidElement),
        }
    }
}
