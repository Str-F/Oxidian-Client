use crate::protocol::packets::configuration::clientbound::disconnect::DisconnectConfigurationClientboundPacket;
use crate::protocol::packets::configuration::clientbound::finish_configuration::FinishConfigurationPacket;
use crate::protocol::packets::configuration::clientbound::keep_alive_configuration::KeepAliveClientboundConfigurationPacket;
use crate::protocol::packets::configuration::clientbound::known_packs::KnownPacksClientboundPacket;
use crate::protocol::packets::configuration::clientbound::registry_data::RegistryDataClientboundPacket;
use crate::protocol::packets::configuration::clientbound::update_tags::UpdateTagsClientboundPacket;
use crate::protocol::packets::login::clientbound::login_compression::SetCompressionClientboundPacket;
use crate::protocol::packets::login::clientbound::login_disconnect::DisconnectLoginClientboundPacket;
use crate::protocol::packets::login::clientbound::login_success::LoginSuccessPacket;
use crate::protocol::packets::play::clientbound::disconnect::DisconnectPlayClientboundPacket;
use crate::protocol::packets::play::clientbound::keep_alive_play::KeepAliveClientboundPlayPacket;
use crate::protocol::packets::play::clientbound::login::LoginClientboundPacket;
use crate::protocol::packets::play::clientbound::player_position::SynchronizePlayerClientboundPacket;
use bytes::BytesMut;
use log::trace;

use crate::protocol::state::ConnectionState;

pub struct PacketDispatcher {}

#[derive(Debug)]
pub enum Event {
    DisconnectLogin {
        packet: DisconnectLoginClientboundPacket,
    },
    EncryptionRequest,
    LoginSuccess {
        packet: LoginSuccessPacket,
    },
    SetCompression {
        packet: SetCompressionClientboundPacket,
    },
    LoginPluginRequest,
    CookieRequest,
    PluginMessage,
    FinishConfiguration {
        packet: FinishConfigurationPacket,
    },
    FeatureFlags,
    KnownPacks {
        packet: KnownPacksClientboundPacket,
    },
    RegistryData {
        packet: RegistryDataClientboundPacket,
    },
    UpdateTags {
        packet: UpdateTagsClientboundPacket,
    },
    KeepAliveConfiguration {
        packet: KeepAliveClientboundConfigurationPacket,
    },
    DisconnectConfiguration {
        packet: DisconnectConfigurationClientboundPacket,
    },

    //play state packets
    KeepAlivePlay {
        packet: KeepAliveClientboundPlayPacket,
    },
    Login {
        packet: LoginClientboundPacket,
    },
    SynchronizePlayerPosition {
        packet: SynchronizePlayerClientboundPacket,
    },
    DisconnectPlay {
        packet: DisconnectPlayClientboundPacket,
    },
}

#[derive(Debug)]
pub enum Error {
    UnknownPacket,
    InvalidState,
    FailedDecode,
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
            ConnectionState::Login => match id {
                0 => Ok(Event::DisconnectLogin {
                    packet: DisconnectLoginClientboundPacket::decode(data)
                        .map_err(|_| Error::FailedDecode)?,
                }),
                1 => Ok(Event::EncryptionRequest),
                2 => Ok(Event::LoginSuccess {
                    packet: LoginSuccessPacket::decode(data).map_err(|_| Error::FailedDecode)?,
                }),
                3 => Ok(Event::SetCompression {
                    packet: SetCompressionClientboundPacket::decode(data)
                        .map_err(|_| Error::FailedDecode)?,
                }),
                4 => Ok(Event::LoginPluginRequest),
                5 => Ok(Event::CookieRequest),
                _ => {
                    trace!("Unknown login state packet: id: {}, data: {:?}", id, data);
                    Err(Error::UnknownPacket)
                }
            },
            ConnectionState::Configuration => match id {
                1 => Ok(Event::PluginMessage),
                2 => Ok(Event::DisconnectConfiguration {
                    packet: DisconnectConfigurationClientboundPacket::decode(data)
                        .map_err(|_| Error::FailedDecode)?,
                }),
                3 => Ok(Event::FinishConfiguration {
                    packet: FinishConfigurationPacket,
                }),
                4 => Ok(Event::KeepAliveConfiguration {
                    packet: KeepAliveClientboundConfigurationPacket::decode(data)
                        .map_err(|_| Error::FailedDecode)?,
                }),
                7 => Ok(Event::RegistryData {
                    packet: RegistryDataClientboundPacket::decode(data)
                        .map_err(|_| Error::FailedDecode)?,
                }),
                12 => Ok(Event::FeatureFlags),
                13 => Ok(Event::UpdateTags {
                    packet: UpdateTagsClientboundPacket::decode(data)
                        .map_err(|_| Error::FailedDecode)?,
                }),
                14 => Ok(Event::KnownPacks {
                    packet: KnownPacksClientboundPacket::decode(data)
                        .map_err(|_| Error::FailedDecode)?,
                }),
                _ => {
                    trace!(
                        "Unknown configuration state packet: id: {}, data: {:?}",
                        id, data
                    );
                    Err(Error::UnknownPacket)
                }
            },
            ConnectionState::Play => match id {
                32 => Ok(Event::DisconnectPlay {
                    packet: DisconnectPlayClientboundPacket::decode(data)
                        .map_err(|_| Error::FailedDecode)?,
                }),
                44 => Ok(Event::KeepAlivePlay {
                    packet: KeepAliveClientboundPlayPacket::decode(data)
                        .map_err(|_| Error::FailedDecode)?,
                }),
                49 => Ok(Event::Login {
                    packet: LoginClientboundPacket::decode(data)
                        .map_err(|_| Error::FailedDecode)?,
                }),
                72 => Ok(Event::SynchronizePlayerPosition {
                    packet: SynchronizePlayerClientboundPacket::decode(data)
                        .map_err(|_| Error::FailedDecode)?,
                }),
                _ => {
                    trace!("Unknown play state packet: id: {}, data: {:?}", id, data);
                    Err(Error::UnknownPacket)
                }
            },
            _ => Err(Error::InvalidState),
        }
    }
}
