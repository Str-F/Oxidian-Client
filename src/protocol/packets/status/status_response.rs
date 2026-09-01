use bytes::BytesMut;

use crate::protocol::{
    state::ConnectionState, traits::packet::ClientboundPacket, types::mcstring::McString,
};

#[derive(Debug)]
pub struct StatusResponsePacket {
    pub json_response: String,
}

impl StatusResponsePacket {
    pub fn decode(bytes: &mut BytesMut) -> Option<Self> {
        let json = McString::decode(bytes);
        Some(Self {
            json_response: json.ok()?.0,
        })
    }
}

impl ClientboundPacket for StatusResponsePacket {
    fn state(&self) -> crate::protocol::state::ConnectionState {
        ConnectionState::Status
    }

    fn id(&self) -> i32 {
        0
    }
}
