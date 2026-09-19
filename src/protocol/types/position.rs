use bytes::{Buf, BufMut, BytesMut};

use crate::protocol::error::ProtocolError;

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position {
    pub fn decode(data: &mut BytesMut) -> Result<Self, ProtocolError> {
        if data.len() < 8 {
            return Err(ProtocolError::UnexpectedEof);
        }

        let value = data.get_i64();
        let x = (value >> 38) as i32;
        let z = ((value << 26) >> 38) as i32;
        let y = ((value << 52) >> 52) as i32;

        Ok(Position { x, y, z })
    }

    pub fn encode(buffer: &mut BytesMut, position: &Position) {
        let value = ((position.x as i64 & 0x3FFFFFF) << 38)
            | ((position.z as i64 & 0x3FFFFFF) << 12)
            | (position.y as i64 & 0xFFF);
        buffer.put_i64(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_encode_normal() {
        let mut buffer = BytesMut::new();
        let position = Position {
            x: 15687927,
            y: 52,
            z: 7223827,
        };
        Position::encode(&mut buffer, &position);
        let decoded = Position::decode(&mut buffer).unwrap();
        assert_eq!(position.x, decoded.x);
        assert_eq!(position.y, decoded.y);
        assert_eq!(position.z, decoded.z);
    }

    #[test]
    fn position_encode_negative() {
        let mut buffer = BytesMut::new();
        let position = Position {
            x: -24891302,
            y: -45,
            z: -13110455,
        };
        Position::encode(&mut buffer, &position);
        let decoded = Position::decode(&mut buffer).unwrap();
        assert_eq!(position.x, decoded.x);
        assert_eq!(position.y, decoded.y);
        assert_eq!(position.z, decoded.z);
    }
}
