use crate::protocol::packets::play::serverbound::accept_teleportation::ConfirmTeleportationServerboundPacket;
use crate::protocol::packets::play::serverbound::keep_alive_play::KeepAliveServerboundPlayPacket;
use crate::protocol::packets::{
    configuration::serverbound::{
        acknowledge_finish_configuration::AcknowledgeFinishConfigurationPacket,
        keep_alive_configuration::KeepAliveServerboundConfigurationPacket,
        known_packs::KnownPacksServerboundPacket,
    },
    login::serverbound::login_acknowledged::LoginAcknowledgedPacket,
};

pub enum NetworkCommand {
    SendLoginAcknowledgedPacket(LoginAcknowledgedPacket),
    SendKnownPacksPacket(KnownPacksServerboundPacket),
    SendAcknowledgeFinishConfigurationPacket(AcknowledgeFinishConfigurationPacket),
    SendKeepAliveConfigurationPacket(KeepAliveServerboundConfigurationPacket),
    SendKeepAlivePlayPacket(KeepAliveServerboundPlayPacket),
    SendConfirmTeleportationPacket(ConfirmTeleportationServerboundPacket),
}
