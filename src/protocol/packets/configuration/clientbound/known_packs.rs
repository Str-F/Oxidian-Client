use crate::protocol::{
    error::ProtocolError,
    state::ConnectionState,
    traits::packet::ClientboundPacket,
    types::{known_packs::KnownPacks as KnownPacksType, mcstring::McString},
    varint,
};
use bytes::{Buf, BytesMut};

#[derive(Debug)]
pub struct KnownPacksClientboundPacket {
    pub packs: Vec<KnownPacksType>,
}

impl ClientboundPacket for KnownPacksClientboundPacket {
    fn state() -> crate::protocol::state::ConnectionState {
        ConnectionState::Configuration
    }

    fn id() -> i32 {
        14
    }
}

impl KnownPacksClientboundPacket {
    pub fn decode(data: &mut BytesMut) -> Result<Self, ProtocolError> {
        let (length, length_size) = varint::decode(data)?;
        data.advance(length_size);

        let mut packs = Vec::new();

        for _ in 0..length {
            let namespace = McString::decode(data)?.0;
            let id = McString::decode(data)?.0;
            let version = McString::decode(data)?.0;

            packs.push(KnownPacksType {
                namespace,
                id,
                version,
            });
        }

        Ok(Self { packs })
    }
}
