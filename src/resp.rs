pub mod parser;
pub mod server;
pub mod types;
pub mod commands;

use crate::resp::types::*;
use crate::resp::parser::{RespElementConstructor, RespParseError};

#[derive(Debug)]
pub enum RespElement {
    Array(RespArray),
    SimpleString(RespSimpleString),
    BulkString(RespBulkString),
}

impl RespElementConstructor for RespElement {
    fn from_byte_slice(slice: &[u8]) -> Result<(Self, &[u8]), RespParseError> {
        match slice.first() {
            Some(b'*') => RespArray::from_byte_slice(slice).map(| (a, r) | (RespElement::Array(a), r)),
            Some(b'+') => RespSimpleString::from_byte_slice(slice).map(| (s, r) | (RespElement::SimpleString(s), r)),
            Some(b'$') => RespBulkString::from_byte_slice(slice).map(| (b, r) | (RespElement::BulkString(b), r)),
            Some(_) => Err(RespParseError::UnknownTypePrefix),
            None => Err(RespParseError::UnexpectedEof),
        }
    }
}
