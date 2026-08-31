use bytes::BytesMut;

use crate::protocol::state::ConnectionState;
use crate::protocol::{traits::packet::ClientboundPacket, types::game_profile::GameProfile};

#[derive(Debug)]
pub struct LoginSuccessPacket {
    pub profile: GameProfile,
    pub session_id: uuid::Uuid,
}

impl ClientboundPacket for LoginSuccessPacket {
    fn state(&self) -> crate::protocol::state::ConnectionState {
        ConnectionState::Login
    }

    fn id(&self) -> i32 {
        2
    }
}

impl LoginSuccessPacket {
    pub fn decode(data: &mut BytesMut) -> Option<Self> {
        if data.len() < 16 {
            return None;
        }

        let profile = GameProfile::decode(data).ok()?;
        let session_id = uuid::Uuid::from_slice(&data.split_to(16)).ok()?;

        Some(Self {
            profile,
            session_id,
        })
    }
}
