use crate::protocol::{state::ConnectionState, traits::packet::ServerboundPacket, varint};
use bytes::BytesMut;

#[derive(Debug)]
pub struct ConfirmTeleportationServerboundPacket {
    teleport_id: i32,
}

impl ConfirmTeleportationServerboundPacket {
    pub fn new(teleport_id: i32) -> Self {
        Self { teleport_id }
    }
}

impl ServerboundPacket for ConfirmTeleportationServerboundPacket {
    fn state() -> ConnectionState {
        ConnectionState::Play
    }

    fn id() -> i32 {
        0
    }

    fn encode_data(&self) -> BytesMut {
        let mut buffer = BytesMut::new();
        varint::encode(&mut buffer, self.teleport_id);
        buffer
    }
}
