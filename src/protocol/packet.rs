use bytes::Buf;
use std::{io::Error, usize};

use crate::protocol::varint;
use bytes::BytesMut;

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

pub fn decode(data: &mut BytesMut) -> Result<Option<(i32, BytesMut)>, Error> {
    let (packet_len, varint_len) = match varint::decode(data) {
        Ok(value) => value,
        Err(varint::VarIntError::NotEnoughBytes) => {
            return Ok(None);
        }
        Err(_) => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid VarInt",
            ));
        }
    };
    if data.len() >= varint_len + packet_len as usize {
        let (packet_id, packet_id_len) = varint::decode(&BytesMut::from(
            &data[varint_len..varint_len + packet_len as usize],
        ))
        .map_err(|_| Error::new(std::io::ErrorKind::InvalidData, "Invalid VarInt"))?;
        let packet_data =
            BytesMut::from(&data[varint_len + packet_id_len..varint_len + packet_len as usize]);
        data.advance(varint_len + packet_len as usize);
        return Ok(Some((packet_id, packet_data)));
    }
    Ok(None)
}
