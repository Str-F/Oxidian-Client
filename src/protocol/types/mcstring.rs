use crate::protocol::varint;
use bytes::{Buf, BytesMut};

#[derive(Debug)]
pub enum McStringError {
    InvalidVarInt,
    NotEnoughBytes,
    InvalidUtf8,
}

impl std::fmt::Display for McStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            McStringError::InvalidVarInt => write!(f, "Invalid VarInt"),
            McStringError::NotEnoughBytes => write!(f, "Not enough bytes to decode string"),
            McStringError::InvalidUtf8 => write!(f, "Invalid UTF-8 string"),
        }
    }
}

impl std::error::Error for McStringError {}

pub struct McString(pub String);

impl McString {
    pub fn decode(bytes: &mut BytesMut) -> Result<Self, McStringError> {
        let (length, length_size) =
            varint::decode(bytes).map_err(|_| McStringError::InvalidVarInt)?;

        bytes.advance(length_size);

        if bytes.len() < length as usize {
            return Err(McStringError::NotEnoughBytes);
        }

        let string_bytes = bytes.split_to(length as usize);

        let string =
            String::from_utf8(string_bytes.to_vec()).map_err(|_| McStringError::InvalidUtf8)?;

        Ok(McString(string))
    }

    pub fn encode(buffer: &mut BytesMut, string: &String) {
        varint::encode(buffer, string.len() as i32);
        buffer.extend_from_slice(string.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mcstring_encode_normal() {
        let mut buffer = BytesMut::new();
        let value = "teststring".to_string();
        McString::encode(&mut buffer, &value);
        let decoded = McString::decode(&mut buffer).unwrap().0;
        assert_eq!(value, decoded);
    }

    #[test]
    fn mcstring_encode_empty() {
        let mut buffer = BytesMut::new();
        let value = "".to_string();
        McString::encode(&mut buffer, &value);
        let decoded = McString::decode(&mut buffer).unwrap().0;
        assert_eq!(value, decoded);
    }

    #[test]
    fn mcstring_encode_not_enough_bytes() {
        let mut buffer = BytesMut::new();
        let value = "teststring".to_string();
        McString::encode(&mut buffer, &value);
        buffer.truncate(value.len() - 1);
        let result = McString::decode(&mut buffer);
        assert!(matches!(result, Err(McStringError::NotEnoughBytes)));
    }

    #[test]
    fn mcstring_encode_invalid_utf8() {
        let mut buffer = BytesMut::new();
        let invalid_utf8 = vec![0xff, 0xfe, 0xfd];
        varint::encode(&mut buffer, invalid_utf8.len() as i32);
        buffer.extend_from_slice(&invalid_utf8);
        let result = McString::decode(&mut buffer);
        assert!(matches!(result, Err(McStringError::InvalidUtf8)));
    }
}
