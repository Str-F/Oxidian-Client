use bytes::{Buf, BytesMut};

use crate::protocol::{state::ConnectionState, traits::packet::ClientboundPacket};

#[derive(Debug)]
pub struct PongResponsePacket {
    pub timestamp: i64,
}

impl PongResponsePacket {
    pub fn decode(data: &mut BytesMut) -> Option<Self> {
        if data.len() < 8 {
            return None;
        }

        let timestamp = data.get_i64();
        Some(Self { timestamp })
    }
}

impl ClientboundPacket for PongResponsePacket {
    fn state(&self) -> ConnectionState {
        ConnectionState::Status
    }

    fn id(&self) -> i32 {
        1
    }
}
