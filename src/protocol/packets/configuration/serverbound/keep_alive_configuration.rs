use bytes::BytesMut;

use crate::protocol::state::ConnectionState;
use crate::protocol::traits::packet::ServerboundPacket;

#[derive(Debug)]
pub struct KeepAliveServerboundConfigurationPacket {
    id: i64,
}

impl KeepAliveServerboundConfigurationPacket {
    pub fn new(id: i64) -> Self {
        Self { id }
    }
}

impl ServerboundPacket for KeepAliveServerboundConfigurationPacket {
    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn id() -> i32 {
        4
    }

    fn encode_data(&self) -> BytesMut {
        let mut buffer = BytesMut::new();
        buffer.extend_from_slice(&self.id.to_be_bytes());
        buffer
    }
}
