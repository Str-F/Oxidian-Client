use bytes::BytesMut;

use crate::protocol::error::ProtocolError;
use crate::protocol::state::ConnectionState;
use crate::protocol::{traits::packet::ClientboundPacket, types::game_profile::GameProfile};

#[derive(Debug)]
pub struct LoginSuccessPacket {
    pub profile: GameProfile,
    pub session_id: uuid::Uuid,
}

impl ClientboundPacket for LoginSuccessPacket {
    fn state() -> crate::protocol::state::ConnectionState {
        ConnectionState::Login
    }

    fn id() -> i32 {
        2
    }
}

impl LoginSuccessPacket {
    pub fn decode(data: &mut BytesMut) -> Result<Self, ProtocolError> {
        if data.len() < 16 {
            return Err(ProtocolError::UnexpectedEof);
        }

        let profile = GameProfile::decode(data)?;
        let session_id = uuid::Uuid::from_slice(&data.split_to(16))
            .map_err(|_| ProtocolError::InvalidData("Invalid UUID".to_string()))?;

        Ok(Self {
            profile,
            session_id,
        })
    }
}
