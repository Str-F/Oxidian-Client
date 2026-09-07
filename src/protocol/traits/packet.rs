use crate::protocol::state::ConnectionState;
use bytes::BytesMut;

pub trait ServerboundPacket {
    fn state() -> ConnectionState;
    fn id() -> i32;
    fn encode_data(&self) -> BytesMut;
}

pub trait ClientboundPacket {
    fn state() -> ConnectionState;
    fn id() -> i32;
}
