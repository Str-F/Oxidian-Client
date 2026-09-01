use crate::protocol::types::mcstring::McString;
use crate::protocol::{state::ConnectionState, traits::packet::ServerboundPacket};
use bytes::BytesMut;

#[derive(Debug)]
pub struct LoginStartPacket {
    name: String,
    uuid: uuid::Uuid,
}

impl LoginStartPacket {
    pub fn new(name: String, uuid: uuid::Uuid) -> Self {
        Self { name, uuid }
    }
}

impl ServerboundPacket for LoginStartPacket {
    fn state(&self) -> ConnectionState {
        ConnectionState::Login
    }

    fn id(&self) -> i32 {
        0
    }

    fn encode_data(&self) -> BytesMut {
        let mut buffer = BytesMut::new();
        McString::encode(&mut buffer, &self.name);
        buffer.extend_from_slice(self.uuid.as_bytes());
        buffer
    }
}
