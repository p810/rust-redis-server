#[derive(Debug)]
pub enum RespParseError {
    UnexpectedEof,
    UnknownTypePrefix,
    InvalidElement,
}

pub trait RespSerialize {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait RespElementConstructor {
    /// Parses the given byte slice and returns a `Result` that contains either:
    /// 
    ///   - a.) A tuple with an instance of the implementer, as well as a byte
    ///        slice that holds any remaining data (empty otherwise); or,
    ///   - b.) An instance of `RespParseError` if parsing failed
    fn from_byte_slice(slice: &[u8]) -> Result<(Self, &[u8]), RespParseError>
    where
        Self: Sized;
}

/// Reads the given byte slice until CRLF is found, and returns a tuple that
/// contains:
/// 
///   - a.) The current line (from 0 until the first CRLF sequence), and
///   - b.) Any remaining data _after_ the first CRLF (or empty otherwise)
/// 
/// If no CRLF sequence was found in the given byte slice, then the tuple will
/// contain the entirety of `input` in its first element, and an intentionally
/// empty slice as its second.
pub fn read_until_crlf(input: &[u8]) -> Option<(&[u8], &[u8])> {
    let position_of_crlf = input.windows(2)
        .position(| bytes | bytes == b"\r\n");

    match position_of_crlf {
        Some(index) => Some((&input[0..index], &input[(index + 2)..])),
        None => Some((&input[0..], &input[input.len()..])),
    }
}

pub fn get_length_of_current_element(input: &[u8]) -> Result<(usize, &[u8]), RespParseError> {
    let Some(first_line) = read_until_crlf(input) else {
        return Err(RespParseError::InvalidElement);
    };

    let (raw_length, remaining_bytes) = first_line;

    let Ok(length_as_string) = String::from_utf8(raw_length.to_vec()) else {
        return Err(RespParseError::InvalidElement);
    };

    let Ok(length) = length_as_string.parse::<usize>() else {
        return Err(RespParseError::InvalidElement);
    };

    if length == 0 {
        return Err(RespParseError::UnexpectedEof);
    }

    Ok((length, remaining_bytes))
}