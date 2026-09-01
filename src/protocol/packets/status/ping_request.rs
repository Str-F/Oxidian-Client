use std::sync::LazyLock;

use bytes::BytesMut;
use tokio::time::Instant;

use crate::protocol::{state::ConnectionState, traits::packet::ServerboundPacket};

pub static START_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);

#[derive(Debug)]
pub struct PingRequestPacket {
    timestamp: i64,
}

impl PingRequestPacket {
    pub fn new() -> Self {
        Self {
            timestamp: START_TIME.elapsed().as_millis() as i64,
        }
    }
}

impl ServerboundPacket for PingRequestPacket {
    fn state(&self) -> crate::protocol::state::ConnectionState {
        ConnectionState::Status
    }

    fn id(&self) -> i32 {
        1
    }

    fn encode_data(&self) -> bytes::BytesMut {
        let mut buffer = BytesMut::new();
        buffer.extend_from_slice(&self.timestamp.to_be_bytes());
        buffer
    }
}
