use crate::protocol::packets::configuration::clientbound::finish_configuration::FinishConfigurationPacket;
use crate::protocol::packets::configuration::clientbound::known_packs::KnownPacksClientboundPacket;
use crate::protocol::packets::login::login_success::LoginSuccessPacket;
use crate::protocol::packets::status::pong_response::PongResponsePacket;
use crate::protocol::packets::status::status_response::StatusResponsePacket;
use bytes::BytesMut;

use crate::protocol::state::ConnectionState;

pub struct PacketDispatcher {}

#[derive(Debug)]
pub enum Event {
    StatusResponse { packet: StatusResponsePacket },
    PongResponse { packet: PongResponsePacket },

    LoginDisconnect,
    EncryptionRequest,
    LoginSuccess { packet: LoginSuccessPacket },
    SetCompression,
    LoginPluginRequest,
    CookieRequest,

    PluginMessage,
    FinishConfiguration { packet: FinishConfigurationPacket },
    FeatureFlags,
    KnownPacks { packet: KnownPacksClientboundPacket },
}

#[derive(Debug)]
pub enum Error {
    UnknownPacket,
    InvalidState,
}

impl PacketDispatcher {
    pub fn new() -> Self {
        Self {}
    }

    pub fn dispatch(
        &self,
        state: ConnectionState,
        id: i32,
        data: &mut BytesMut,
    ) -> Result<Event, Error> {
        match state {
            ConnectionState::Status => match id {
                0 => Ok(Event::StatusResponse {
                    packet: StatusResponsePacket::decode(data).ok_or(Error::UnknownPacket)?,
                }),
                1 => Ok(Event::PongResponse {
                    packet: PongResponsePacket::decode(data).ok_or(Error::UnknownPacket)?,
                }),
                _ => Err(Error::UnknownPacket),
            },
            ConnectionState::Login => match id {
                0 => Ok(Event::LoginDisconnect),
                1 => Ok(Event::EncryptionRequest),
                2 => Ok(Event::LoginSuccess {
                    packet: LoginSuccessPacket::decode(data).ok_or(Error::UnknownPacket)?,
                }),
                3 => Ok(Event::SetCompression),
                4 => Ok(Event::LoginPluginRequest),
                5 => Ok(Event::CookieRequest),
                _ => Err(Error::UnknownPacket),
            },
            ConnectionState::Configuration => match id {
                1 => Ok(Event::PluginMessage),
                3 => Ok(Event::FinishConfiguration {
                    packet: FinishConfigurationPacket,
                }),
                12 => Ok(Event::FeatureFlags),
                14 => Ok(Event::KnownPacks {
                    packet: KnownPacksClientboundPacket::decode(data)
                        .ok_or(Error::UnknownPacket)?,
                }),
                _ => {
                    println!("Unknwon Configuration packet: id: {}, data: {:?}", id, data);
                    Err(Error::UnknownPacket)
                }
            },
            _ => Err(Error::InvalidState),
        }
    }
}
