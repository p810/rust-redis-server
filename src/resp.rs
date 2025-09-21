pub mod parser;
pub mod types;
pub mod commands;

use crate::resp::types::*;
use crate::resp::parser::{RespElementConstructor, RespParseError, RespSerialize};

pub const RESP_DELIMITER: &[u8] = b"\r\n";
pub const RESP_OK: &[u8] = b"+OK\r\n";
pub const RESP_EMPTY_STRING: &[u8; 5] = b"$-1\r\n";

#[derive(Debug)]
pub enum RespElement {
    Array(RespArray),
    SimpleString(RespSimpleString),
    BulkString(RespBulkString),
    Integer(RespInteger),
}

impl RespElement {
    pub fn new_array(elements: Vec<RespElement>) -> RespElement {
        RespElement::Array(RespArray::new(elements))
    }

    pub fn new_bulk_string(value: &[u8]) -> RespElement {
        RespElement::BulkString(RespBulkString::new(value))
    }
}

impl RespSerialize for RespElement {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            RespElement::Array(a) => a.to_bytes(),
            RespElement::BulkString(b) => b.to_bytes(),
            RespElement::Integer(i) => i.to_bytes(),
            RespElement::SimpleString(s) => s.to_bytes(),
        }
    }
}

impl RespElementConstructor for RespElement {
    fn from_byte_slice(slice: &[u8]) -> Result<(Self, &[u8]), RespParseError> {
        match slice.first() {
            Some(b'*') => RespArray::from_byte_slice(slice).map(| (a, r) | (RespElement::Array(a), r)),
            Some(b'+') => RespSimpleString::from_byte_slice(slice).map(| (s, r) | (RespElement::SimpleString(s), r)),
            Some(b'$') => RespBulkString::from_byte_slice(slice).map(| (b, r) | (RespElement::BulkString(b), r)),
            Some(b':') => RespInteger::from_byte_slice(slice).map(| (i, r) | (RespElement::Integer(i), r)),
            Some(_) => Err(RespParseError::UnknownTypePrefix),
            None => Err(RespParseError::UnexpectedEof),
        }
    }
}
