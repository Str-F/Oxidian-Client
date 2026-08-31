use crate::protocol::{state::ConnectionState, traits::packet::ServerboundPacket};
use bytes::BytesMut;

#[derive(Debug)]
pub struct AcknowledgeFinishConfigurationPacket;

impl ServerboundPacket for AcknowledgeFinishConfigurationPacket {
    fn state(&self) -> ConnectionState {
        ConnectionState::Configuration
    }

    fn id(&self) -> i32 {
        3
    }

    fn encode_data(&self) -> bytes::BytesMut {
        BytesMut::new()
    }
}
