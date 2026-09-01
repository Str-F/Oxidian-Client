use bytes::BytesMut;

use crate::protocol::{state::ConnectionState, traits::packet::ServerboundPacket};

#[derive(Debug)]
pub struct StatusRequestPacket;

impl StatusRequestPacket {
    pub fn new() -> Self {
        Self {}
    }
}

impl ServerboundPacket for StatusRequestPacket {
    fn state(&self) -> crate::protocol::state::ConnectionState {
        ConnectionState::Status
    }

    fn id(&self) -> i32 {
        0
    }

    fn encode_data(&self) -> bytes::BytesMut {
        BytesMut::new()
    }
}
