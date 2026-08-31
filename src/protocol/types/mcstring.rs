use crate::protocol::varint;
use bytes::{Buf, BytesMut};
use std::io::Error;

pub struct McString(pub String);

impl McString {
    pub fn decode(bytes: &mut BytesMut) -> Result<Self, Error> {
        let (length, length_size) = varint::decode(bytes)
            .map_err(|_| Error::new(std::io::ErrorKind::InvalidData, "Invalid VarInt"))?;

        bytes.advance(length_size);

        if bytes.len() < length as usize {
            return Err(Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Not enough bytes to decode string",
            ));
        }

        let string_bytes = bytes.split_to(length as usize);

        let string = String::from_utf8(string_bytes.to_vec())
            .map_err(|_| Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

        Ok(McString(string))
    }

    pub fn encode(buffer: &mut BytesMut, string: &String) {
        varint::encode(buffer, string.len() as i32);
        buffer.extend_from_slice(string.as_bytes());
    }
}
