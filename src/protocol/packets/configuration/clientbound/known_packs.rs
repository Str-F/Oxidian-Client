use crate::protocol::{
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
    fn state(&self) -> crate::protocol::state::ConnectionState {
        ConnectionState::Configuration
    }

    fn id(&self) -> i32 {
        14
    }
}

impl KnownPacksClientboundPacket {
    pub fn decode(data: &mut BytesMut) -> Option<Self> {
        let (length, length_size) = varint::decode(data).ok()?;
        data.advance(length_size);

        let mut packs = Vec::new();

        for _ in 0..length {
            let namespace = McString::decode(data).ok()?.0;
            let id = McString::decode(data).ok()?.0;
            let version = McString::decode(data).ok()?.0;

            packs.push(KnownPacksType {
                namespace,
                id,
                version,
            });
        }

        Some(Self { packs })
    }
}
