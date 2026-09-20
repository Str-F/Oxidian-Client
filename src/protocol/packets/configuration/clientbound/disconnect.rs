use crate::protocol::{
    error::ProtocolError, state::ConnectionState, traits::packet::ClientboundPacket,
};
use bytes::{Buf, BytesMut};
use fastnbt::DeOpts;

#[derive(Debug)]
pub struct DisconnectConfigurationClientboundPacket {
    pub reason: fastnbt::Value,
}

impl DisconnectConfigurationClientboundPacket {
    pub fn decode(data: &mut BytesMut) -> Result<Self, ProtocolError> {
        let mut slice = &data[..];
        let len_before = slice.len();

        let reason = fastnbt::from_reader_with_opts(&mut slice, DeOpts::network_nbt())
            .map_err(|e| ProtocolError::InvalidData(e.to_string()))?;

        let bytes_read = len_before - slice.len();
        data.advance(bytes_read);
        Ok(Self { reason })
    }
}

impl ClientboundPacket for DisconnectConfigurationClientboundPacket {
    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn id() -> i32 {
        2
    }
}
