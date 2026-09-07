use crate::protocol::{state::ConnectionState, traits::packet::ServerboundPacket};
use bytes::BytesMut;

#[derive(Debug)]
pub struct LoginAcknowledgedPacket;

impl LoginAcknowledgedPacket {
    pub fn new() -> Self {
        Self {}
    }
}

impl ServerboundPacket for LoginAcknowledgedPacket {
    fn state() -> ConnectionState {
        ConnectionState::Login
    }

    fn id() -> i32 {
        3
    }

    fn encode_data(&self) -> BytesMut {
        BytesMut::new()
    }
}
