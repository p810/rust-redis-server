use crate::resp::parser::{
    RespSerialize,
    RespDeserialize,
    RespParseError,
    read_until_crlf,
};

#[derive(Debug)]
pub struct RespSimpleString {
    pub value: String,
}

impl RespSerialize for RespSimpleString {
    fn to_bytes(&self) -> Vec<u8> {
        let mut acc = vec![b'+'];

        acc.extend(self.value.as_bytes());

        acc
    }
}

impl RespDeserialize for RespSimpleString {
    fn from_byte_slice(input: &[u8]) -> Result<(Self, &[u8]), RespParseError> {
        if input[0] != b'+' {
            return Err(RespParseError::UnknownTypePrefix);
        }

        match read_until_crlf(&input[1..]) {
            // to do: don't allow special chars like crlf in this
            Some((data, remaining_bytes)) => {
                let Ok(value) = String::from_utf8(data.to_vec()) else {
                    return Err(RespParseError::InvalidElement);
                };

                Ok((RespSimpleString { value }, remaining_bytes))
            }
            None => Err(RespParseError::InvalidElement),
        }
    }
}
