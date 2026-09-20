use crate::protocol::{
    error::ProtocolError, state::ConnectionState, traits::packet::ClientboundPacket,
    types::mcstring::McString,
};
use bytes::BytesMut;

#[derive(Debug)]
pub struct DisconnectLoginClientboundPacket {
    pub reason: String,
}

impl DisconnectLoginClientboundPacket {
    pub fn decode(data: &mut BytesMut) -> Result<Self, ProtocolError> {
        Ok(Self {
            reason: McString::decode(data)?.0,
        })
    }
}

impl ClientboundPacket for DisconnectLoginClientboundPacket {
    fn state() -> ConnectionState {
        ConnectionState::Login
    }

    fn id() -> i32 {
        0
    }
}
