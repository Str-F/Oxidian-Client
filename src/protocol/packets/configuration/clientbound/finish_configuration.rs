use crate::protocol::{state::ConnectionState, traits::packet::ClientboundPacket};

#[derive(Debug)]
pub struct FinishConfigurationPacket;

impl ClientboundPacket for FinishConfigurationPacket {
    fn state(&self) -> ConnectionState {
        ConnectionState::Configuration
    }

    fn id(&self) -> i32 {
        3
    }
}
