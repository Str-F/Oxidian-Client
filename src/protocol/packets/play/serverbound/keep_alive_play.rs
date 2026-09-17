use bytes::BytesMut;

use crate::protocol::state::ConnectionState;
use crate::protocol::traits::packet::ServerboundPacket;

#[derive(Debug)]
pub struct KeepAliveServerboundPlayPacket {
    id: i64,
}

impl KeepAliveServerboundPlayPacket {
    pub fn new(id: i64) -> Self {
        Self { id }
    }
}

impl ServerboundPacket for KeepAliveServerboundPlayPacket {
    fn state() -> ConnectionState {
        ConnectionState::Play
    }

    fn id() -> i32 {
        28
    }

    fn encode_data(&self) -> BytesMut {
        let mut buffer = BytesMut::new();
        buffer.extend_from_slice(&self.id.to_be_bytes());
        buffer
    }
}
