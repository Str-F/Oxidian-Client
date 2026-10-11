use bytes::Buf;
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use std::error::Error;
use std::fmt::Display;
use std::io::{ErrorKind, Read, Write};

use crate::protocol::varint::{self, VarIntError};
use bytes::BytesMut;

#[derive(Debug)]
pub enum PacketError {
    InvalidVarInt,
    SizeMismatch,
    Io(std::io::Error),
}

impl Display for PacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketError::InvalidVarInt => write!(f, "Invalid VarInt"),
            PacketError::SizeMismatch => write!(f, "Size Mismatch"),
            PacketError::Io(err) => write!(f, "IO Error: {}", err),
        }
    }
}

impl Error for PacketError {}

impl From<PacketError> for std::io::Error {
    fn from(err: PacketError) -> Self {
        std::io::Error::new(ErrorKind::InvalidData, err)
    }
}

impl From<std::io::Error> for PacketError {
    fn from(err: std::io::Error) -> Self {
        PacketError::Io(err)
    }
}

pub fn encode(
    id: i32,
    data: &[u8],
    compression_threshold: Option<i32>,
) -> Result<BytesMut, PacketError> {
    let mut inner = BytesMut::new();
    varint::encode(&mut inner, id);
    inner.extend_from_slice(data);

    let packet_body = match compression_threshold {
        None => inner,
        Some(threshold) if threshold >= 0 && inner.len() >= threshold as usize => {
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&inner)?;
            let compressed_data = encoder.finish()?;

            let mut buf = BytesMut::with_capacity(compressed_data.len() + 5);
            varint::encode(&mut buf, inner.len() as i32);
            buf.extend_from_slice(&compressed_data);
            buf
        }
        Some(_) => {
            let mut buf = BytesMut::with_capacity(inner.len() + 1);
            varint::encode(&mut buf, 0);
            buf.extend_from_slice(&inner);
            buf
        }
    };

    let mut output = BytesMut::with_capacity(packet_body.len() + 5);
    varint::encode(&mut output, packet_body.len() as i32);
    output.extend_from_slice(&packet_body);
    Ok(output)
}

pub fn decode(
    data: &mut BytesMut,
    compression_threshold: Option<i32>,
) -> Result<Option<(i32, BytesMut)>, PacketError> {
    let (packet_len, varint_len) = match varint::decode(data) {
        Ok(value) => value,
        Err(VarIntError::NotEnoughBytes) => return Ok(None),
        Err(_) => return Err(PacketError::InvalidVarInt),
    };

    if packet_len < 0 {
        return Err(PacketError::InvalidVarInt);
    }

    if data.len() < varint_len + packet_len as usize {
        return Ok(None);
    }

    let frame = &data[varint_len..varint_len + packet_len as usize];

    let packet_body = if let Some(_) = compression_threshold {
        let (data_len, data_varint_len) =
            varint::decode(&BytesMut::from(frame)).map_err(|_| PacketError::InvalidVarInt)?;

        if data_len < 0 || data_len > 8_388_608 {
            return Err(PacketError::SizeMismatch);
        }

        if data_len != 0 {
            let compressed_data = &frame[data_varint_len..];
            let mut uncompressed_data = BytesMut::with_capacity(data_len as usize);
            uncompressed_data.resize(data_len as usize, 0);
            let mut decoder = ZlibDecoder::new(compressed_data);
            decoder
                .read_exact(&mut uncompressed_data)
                .map_err(|_| PacketError::SizeMismatch)?;
            let mut extra = [0u8; 1];
            if decoder
                .read(&mut extra)
                .map_err(|_| PacketError::SizeMismatch)?
                != 0
            {
                return Err(PacketError::SizeMismatch);
            }
            uncompressed_data
        } else {
            BytesMut::from(&frame[data_varint_len..])
        }
    } else {
        BytesMut::from(frame)
    };

    let (packet_id, packet_id_len) =
        varint::decode(&packet_body).map_err(|_| PacketError::InvalidVarInt)?;
    let packet_data = BytesMut::from(&packet_body[packet_id_len..]);

    data.advance(varint_len + packet_len as usize);
    Ok(Some((packet_id, packet_data)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_encode_normal() {
        let packet_id = 1;
        let packet_data = b"teststring";
        let mut encoded_packet = encode(packet_id, packet_data, None).unwrap();
        let decoded_packet = decode(&mut encoded_packet, None).unwrap().unwrap();
        assert_eq!(decoded_packet.0, packet_id);
        assert_eq!(&decoded_packet.1[..], packet_data);
    }

    #[test]
    fn packet_encode_incomplete_packet() {
        let packet_id = 1;
        let packet_data = b"teststring";
        let mut encoded_packet = encode(packet_id, packet_data, None).unwrap();
        encoded_packet.truncate(encoded_packet.len() - 1);
        let decoded_packet = decode(&mut encoded_packet, None);
        assert!(matches!(decoded_packet, Ok(None)));
    }

    #[test]
    fn packet_encode_multiple_packets() {
        let packet_id1 = 1;
        let packet_data1 = b"teststring1";
        let packet_id2 = 2;
        let packet_data2 = b"teststring2";
        let mut encoded_packet1 = encode(packet_id1, packet_data1, None).unwrap();
        let encoded_packet2 = encode(packet_id2, packet_data2, None).unwrap();
        encoded_packet1.extend_from_slice(&encoded_packet2);
        let decoded_packet1 = decode(&mut encoded_packet1, None).unwrap().unwrap();
        assert_eq!(decoded_packet1.0, packet_id1);
        assert_eq!(&decoded_packet1.1[..], packet_data1);
        let decoded_packet2 = decode(&mut encoded_packet1, None).unwrap().unwrap();
        assert_eq!(decoded_packet2.0, packet_id2);
        assert_eq!(&decoded_packet2.1[..], packet_data2);
    }

    #[test]
    fn packet_encode_compression_normal() {
        let packet_id = 1;
        let packet_data = b"teststring"; // len = 11 (data + id)
        let compression_threshold = 5;
        let mut encoded_packet =
            encode(packet_id, packet_data, Some(compression_threshold)).unwrap();
        let decoded_packet = decode(&mut encoded_packet, Some(compression_threshold))
            .unwrap()
            .unwrap();
        assert_eq!(decoded_packet.0, packet_id);
        assert_eq!(&decoded_packet.1[..], packet_data);
    }

    #[test]
    fn packet_encode_compression_under_threshold() {
        let packet_id = 1;
        let packet_data = b"teststring"; // len = 11 (data + id)
        let compression_threshold = 15;
        let mut encoded_packet =
            encode(packet_id, packet_data, Some(compression_threshold)).unwrap();
        let decoded_packet = decode(&mut encoded_packet, Some(compression_threshold))
            .unwrap()
            .unwrap();
        assert_eq!(decoded_packet.0, packet_id);
        assert_eq!(&decoded_packet.1[..], packet_data);
    }
}
