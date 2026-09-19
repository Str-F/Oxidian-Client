use crate::protocol::{
    error::ProtocolError, state::ConnectionState, traits::packet::ClientboundPacket, varint,
};
use bytes::{Buf, BytesMut};

#[derive(Debug)]
pub struct SynchronizePlayerClientboundPacket {
    pub teleport_id: i32,
    x: f64,
    y: f64,
    z: f64,
    velocity_x: f64,
    velocity_y: f64,
    velocity_z: f64,
    yaw: f32,
    pitch: f32,
    flags: i32,
}

impl SynchronizePlayerClientboundPacket {
    pub fn decode(data: &mut BytesMut) -> Result<Self, ProtocolError> {
        let teleport_id = varint::decode_mut(data)?;
        let x = data.get_f64();
        let y = data.get_f64();
        let z = data.get_f64();
        let velocity_x = data.get_f64();
        let velocity_y = data.get_f64();
        let velocity_z = data.get_f64();
        let yaw = data.get_f32();
        let pitch = data.get_f32();
        let flags = data.get_i32();
        Ok(Self {
            teleport_id,
            x,
            y,
            z,
            velocity_x,
            velocity_y,
            velocity_z,
            yaw,
            pitch,
            flags,
        })
    }
}

impl ClientboundPacket for SynchronizePlayerClientboundPacket {
    fn state() -> ConnectionState {
        ConnectionState::Play
    }

    fn id() -> i32 {
        72
    }
}
