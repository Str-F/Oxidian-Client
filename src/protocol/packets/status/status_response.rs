use bytes::BytesMut;

use crate::protocol::{
    error::ProtocolError, state::ConnectionState, traits::packet::ClientboundPacket,
    types::mcstring::McString,
};

#[derive(Debug)]
pub struct StatusResponsePacket {
    pub json_response: String,
}

impl StatusResponsePacket {
    pub fn decode(bytes: &mut BytesMut) -> Result<Self, ProtocolError> {
        Ok(Self {
            json_response: McString::decode(bytes)?.0,
        })
    }
}

impl ClientboundPacket for StatusResponsePacket {
    fn state() -> crate::protocol::state::ConnectionState {
        ConnectionState::Status
    }

    fn id() -> i32 {
        0
    }
}
