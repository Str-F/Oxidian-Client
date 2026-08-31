use crate::protocol::state::ConnectionState;
use bytes::BytesMut;

pub trait ServerboundPacket {
    fn state(&self) -> ConnectionState;
    fn id(&self) -> i32;
    fn encode_data(&self) -> BytesMut;
}

pub trait ClientboundPacket {
    fn state(&self) -> ConnectionState;
    fn id(&self) -> i32;
}
