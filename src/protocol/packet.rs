use bytes::Buf;
use std::error::Error;
use std::fmt::Display;
use std::io::ErrorKind;
use std::usize;

use crate::protocol::varint;
use bytes::BytesMut;

#[derive(Debug)]
pub enum PacketError {
    InvalidVarInt,
}

impl Display for PacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketError::InvalidVarInt => write!(f, "Invalid VarInt"),
        }
    }
}

impl Error for PacketError {}

impl From<PacketError> for std::io::Error {
    fn from(err: PacketError) -> Self {
        std::io::Error::new(ErrorKind::InvalidData, err)
    }
}

pub fn encode(id: i32, data: &[u8]) -> BytesMut {
    let mut id_buffer = BytesMut::new();
    varint::encode(&mut id_buffer, id);
    let packet_len = id_buffer.len() + data.len();
    let mut output = BytesMut::with_capacity(packet_len + 5); // +5 for maximum size of varint
    varint::encode(&mut output, packet_len as i32);
    output.extend_from_slice(&id_buffer);
    output.extend_from_slice(data);
    output
}

pub fn decode(data: &mut BytesMut) -> Result<Option<(i32, BytesMut)>, PacketError> {
    let (packet_len, varint_len) = match varint::decode(data) {
        Ok(value) => value,
        Err(varint::VarIntError::NotEnoughBytes) => {
            return Ok(None);
        }
        Err(_) => {
            return Err(PacketError::InvalidVarInt);
        }
    };
    if data.len() >= varint_len + packet_len as usize {
        let (packet_id, packet_id_len) = varint::decode(&BytesMut::from(
            &data[varint_len..varint_len + packet_len as usize],
        ))
        .map_err(|_| PacketError::InvalidVarInt)?;
        let packet_data =
            BytesMut::from(&data[varint_len + packet_id_len..varint_len + packet_len as usize]);
        data.advance(varint_len + packet_len as usize);
        return Ok(Some((packet_id, packet_data)));
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_encode_normal() {
        let packet_id = 1;
        let packet_data = b"teststring";
        let mut encoded_packet = encode(packet_id, packet_data);
        let decoded_packet = decode(&mut encoded_packet).unwrap().unwrap();
        assert_eq!(decoded_packet.0, packet_id);
        assert_eq!(&decoded_packet.1[..], packet_data);
    }

    #[test]
    fn packet_encode_incomplete_packet() {
        let packet_id = 1;
        let packet_data = b"teststring";
        let mut encoded_packet = encode(packet_id, packet_data);
        encoded_packet.truncate(encoded_packet.len() - 1);
        let decoded_packet = decode(&mut encoded_packet);
        assert!(matches!(decoded_packet, Ok(None)));
    }

    #[test]
    fn packet_encode_multiple_packets() {
        let packet_id1 = 1;
        let packet_data1 = b"teststring1";
        let packet_id2 = 2;
        let packet_data2 = b"teststring2";
        let mut encoded_packet1 = encode(packet_id1, packet_data1);
        let encoded_packet2 = encode(packet_id2, packet_data2);
        encoded_packet1.extend_from_slice(&encoded_packet2);
        let decoded_packet1 = decode(&mut encoded_packet1).unwrap().unwrap();
        assert_eq!(decoded_packet1.0, packet_id1);
        assert_eq!(&decoded_packet1.1[..], packet_data1);
        let decoded_packet2 = decode(&mut encoded_packet1).unwrap().unwrap();
        assert_eq!(decoded_packet2.0, packet_id2);
        assert_eq!(&decoded_packet2.1[..], packet_data2);
    }
}
