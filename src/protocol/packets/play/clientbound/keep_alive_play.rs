use bytes::Buf;
use bytes::BytesMut;

use crate::protocol::error::ProtocolError;
use crate::protocol::state::ConnectionState;
use crate::protocol::traits::packet::ClientboundPacket;

#[derive(Debug)]
pub struct KeepAliveClientboundPlayPacket {
    pub id: i64,
}

impl KeepAliveClientboundPlayPacket {
    pub fn decode(data: &mut BytesMut) -> Result<Self, ProtocolError> {
        if data.len() < 8 {
            return Err(ProtocolError::UnexpectedEof);
        }

        let id = data.get_i64();
        Ok(Self { id })
    }
}

impl ClientboundPacket for KeepAliveClientboundPlayPacket {
    fn state() -> crate::protocol::state::ConnectionState {
        ConnectionState::Play
    }

    fn id() -> i32 {
        44
    }
}
