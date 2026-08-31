use crate::protocol::{
    state::ConnectionState,
    traits::packet::ServerboundPacket,
    types::{known_packs::KnownPacks as KnownPacksType, mcstring::McString},
    varint,
};
use bytes::BytesMut;

#[derive(Debug)]
pub struct KnownPacksServerboundPacket {
    pub packs: Vec<KnownPacksType>,
}

impl KnownPacksServerboundPacket {
    pub fn new(packs: Vec<KnownPacksType>) -> Self {
        Self { packs }
    }
}

impl ServerboundPacket for KnownPacksServerboundPacket {
    fn state(&self) -> ConnectionState {
        ConnectionState::Configuration
    }

    fn id(&self) -> i32 {
        7
    }

    fn encode_data(&self) -> BytesMut {
        let mut buffer = BytesMut::new();
        varint::encode(&mut buffer, self.packs.len() as i32);

        for pack in &self.packs {
            McString::encode(&mut buffer, &pack.namespace);
            McString::encode(&mut buffer, &pack.id);
            McString::encode(&mut buffer, &pack.version);
        }

        buffer
    }
}
