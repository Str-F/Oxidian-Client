use crate::protocol::{state::ConnectionState, traits::packet::ClientboundPacket};

#[derive(Debug)]
pub struct FinishConfigurationPacket;

impl ClientboundPacket for FinishConfigurationPacket {
    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn id() -> i32 {
        3
    }
}
