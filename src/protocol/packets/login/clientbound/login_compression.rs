use crate::protocol::{
    error::ProtocolError, state::ConnectionState, traits::packet::ClientboundPacket, varint,
};
use bytes::BytesMut;

#[derive(Debug)]
pub struct SetCompressionClientboundPacket {
    pub threshold: i32,
}

impl SetCompressionClientboundPacket {
    pub fn decode(data: &mut BytesMut) -> Result<Self, ProtocolError> {
        let threshold = varint::decode(data)?.0;
        Ok(Self { threshold })
    }
}

impl ClientboundPacket for SetCompressionClientboundPacket {
    fn state() -> ConnectionState {
        ConnectionState::Login
    }

    fn id() -> i32 {
        3
    }
}
