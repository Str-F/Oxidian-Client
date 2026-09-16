use bytes::{Buf, BytesMut};

use crate::protocol::{
    error::ProtocolError, state::ConnectionState, traits::packet::ClientboundPacket,
};

#[derive(Debug)]
pub struct PongResponsePacket {
    pub timestamp: i64,
}

impl PongResponsePacket {
    pub fn decode(data: &mut BytesMut) -> Result<Self, ProtocolError> {
        if data.len() < 8 {
            return Err(ProtocolError::UnexpectedEof);
        }

        let timestamp = data.get_i64();
        Ok(Self { timestamp })
    }
}

impl ClientboundPacket for PongResponsePacket {
    fn state() -> ConnectionState {
        ConnectionState::Status
    }

    fn id() -> i32 {
        1
    }
}
