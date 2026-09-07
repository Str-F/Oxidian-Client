use crate::protocol::packets::{
    configuration::serverbound::{
        acknowledge_finish_configuration::AcknowledgeFinishConfigurationPacket,
        known_packs::KnownPacksServerboundPacket,
    },
    login::login_acknowledged::LoginAcknowledgedPacket,
};

pub enum NetworkCommand {
    SendLoginAcknowledgedPacket(LoginAcknowledgedPacket),
    SendKnownPacksPacket(KnownPacksServerboundPacket),
    SendAcknowledgeFinishConfigurationPacket(AcknowledgeFinishConfigurationPacket),
}
