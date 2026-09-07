use crate::protocol::{state::ConnectionState, traits::packet::ServerboundPacket};
use bytes::BytesMut;

#[derive(Debug)]
pub struct AcknowledgeFinishConfigurationPacket;

impl ServerboundPacket for AcknowledgeFinishConfigurationPacket {
    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn id() -> i32 {
        3
    }

    fn encode_data(&self) -> bytes::BytesMut {
        BytesMut::new()
    }
}
