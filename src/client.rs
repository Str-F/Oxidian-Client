use crate::network::command::NetworkCommand;
use crate::network::manager::NetworkManager;
use crate::protocol::dispatcher::Event;
use crate::protocol::packets::configuration::serverbound::acknowledge_finish_configuration::AcknowledgeFinishConfigurationPacket;
use crate::protocol::packets::configuration::serverbound::keep_alive_configuration::KeepAliveServerboundConfigurationPacket;
use crate::protocol::packets::configuration::serverbound::known_packs::KnownPacksServerboundPacket;
use crate::protocol::packets::login::serverbound::login_acknowledged::LoginAcknowledgedPacket;
use crate::protocol::packets::play::serverbound::accept_teleportation::ConfirmTeleportationServerboundPacket;
use crate::protocol::packets::play::serverbound::keep_alive_play::KeepAliveServerboundPlayPacket;
use crate::protocol::registry::Registry;
use crate::protocol::tags::Tags;
use crate::server::Server;
use log::{debug, error, info, warn};
use tokio::sync::mpsc;

pub const PROTOCOL_VER: i32 = 776;
pub const MC_VER: &str = "26.2";
pub const USERNAME: &str = "OxidianClientUsr";

pub struct Client {
    event_receiver: Option<mpsc::Receiver<Event>>,
    command_sender: Option<mpsc::Sender<NetworkCommand>>,
    registry: Registry,
    tags: Tags,
    running: bool,
}

impl Client {
    pub fn new() -> Self {
        Self {
            event_receiver: None,
            command_sender: None,
            registry: Registry::new(),
            tags: Tags::new(),
            running: true,
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub async fn run(&mut self) {
        self.get_status().await;
        self.connect_server().await;

        while self.running {
            self.update();
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }

    pub async fn get_status(&mut self) {
        let server = Server::new("localhost", 25565);

        match NetworkManager::get_status_server(&server).await {
            Ok(status) => {
                match serde_json::from_str::<serde_json::Value>(&status.0.json_response) {
                    Ok(json) => {
                        let version_name = json["version"]["name"].as_str().unwrap_or("Unknown");
                        let version_protocol = json["version"]["protocol"].as_i64().unwrap_or(0);
                        let players_online = json["players"]["online"].as_u64().unwrap_or(0);
                        let players_max = json["players"]["max"].as_u64().unwrap_or(0);
                        let players_sample: Vec<String> = json["players"]["sample"]
                            .as_array()
                            .map(|a| {
                                a.iter()
                                    .map(|p| p["name"].as_str().unwrap_or("Unknown").to_string())
                                    .collect()
                            })
                            .unwrap_or_default();
                        let description = json["description"]
                            .as_str()
                            .or_else(|| json["description"]["text"].as_str())
                            .unwrap_or("No description");

                        info!(
                            "Server: {}/{} | {}/{} players | {} | sample: {:?}",
                            version_name,
                            version_protocol,
                            players_online,
                            players_max,
                            description,
                            players_sample
                        );
                    }
                    Err(e) => {
                        warn!("Failed to parse status JSON: {}", e);
                    }
                }
                info!("Server ping: {} ms", status.1);
            }
            Err(e) => {
                error!(
                    "Failed to get server status: {}:{} - {}",
                    server.host(),
                    server.port(),
                    e
                );
                self.running = false;
            }
        }
    }

    pub async fn connect_server(&mut self) {
        let server = Server::new("localhost", 25565);
        self.registry = Registry::new();
        self.tags = Tags::new();

        match NetworkManager::join_server(&server).await {
            Ok((command_sender, event_receiver)) => {
                self.command_sender = Some(command_sender);
                self.event_receiver = Some(event_receiver);
                info!("Connected to server: {}:{}", server.host(), server.port());
            }
            Err(e) => {
                error!(
                    "Failed to connect to server: {}:{} - {}",
                    server.host(),
                    server.port(),
                    e
                );
                self.running = false;
            }
        }
    }

    pub fn update(&mut self) {
        let mut events = Vec::new();
        if let Some(receiver) = &mut self.event_receiver {
            loop {
                match receiver.try_recv() {
                    Ok(event) => {
                        events.push(event);
                    }
                    Err(mpsc::error::TryRecvError::Empty) => break,
                    Err(mpsc::error::TryRecvError::Disconnected) => {
                        error!("Event channel disconnected");
                        self.running = false;
                        break;
                    }
                }
            }
        }

        for event in events {
            self.handle_event(event);
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::LoginSuccess { packet } => {
                debug!("Handling packet: {:?}", packet);
                info!(
                    "Logged in as: UUID: {}, Username: {}",
                    packet.profile.uuid, packet.profile.name
                );
                debug!("Session ID: {}", packet.session_id);
                debug!("Profile Properties:");
                for property in packet.profile.properties {
                    debug!(
                        "  Name: {}, Value: {}, Signature: {:?}",
                        property.name, property.value, property.signature
                    );
                }
                let login_ack_packet = LoginAcknowledgedPacket::new();
                if let Some(command_sender) = &self.command_sender {
                    debug!("Sending login acknowledged packet: {:?}", login_ack_packet);
                    if let Err(e) = command_sender.try_send(
                        NetworkCommand::SendLoginAcknowledgedPacket(login_ack_packet),
                    ) {
                        error!("Failed to send login acknowledged packet: {}", e);
                    }
                }
            }

            Event::KnownPacks { packet } => {
                debug!("Received known packs packet: {:?}", packet);
                let known_packs_packet = KnownPacksServerboundPacket::new(packet.packs);
                if let Some(command_sender) = &self.command_sender {
                    debug!("Sending known packs packet: {:?}", known_packs_packet);
                    if let Err(e) = command_sender
                        .try_send(NetworkCommand::SendKnownPacksPacket(known_packs_packet))
                    {
                        error!("Failed to send known packs packet: {}", e);
                    }
                }
            }

            Event::RegistryData { packet } => {
                debug!(
                    "Received registry data packet: {:?} with {} entries",
                    packet.id,
                    packet.entries.len()
                );
                self.registry
                    .add_to_registry(packet.id.clone(), packet.entries);
            }

            Event::UpdateTags { packet } => {
                debug!(
                    "Received update tags packet: {:?} with {} entries",
                    packet
                        .registries
                        .iter()
                        .map(|r| &r.registry)
                        .collect::<Vec<_>>(),
                    packet
                        .registries
                        .iter()
                        .map(|r| r.tags.len())
                        .sum::<usize>()
                );
                for reg in packet.registries {
                    for tag in reg.tags {
                        self.tags
                            .update_tag(reg.registry.clone(), tag.name, tag.entries);
                    }
                }
            }

            Event::FinishConfiguration { packet } => {
                debug!("Received finish configuration packet: {:?}", packet);
                let acknowledge_finish_configuration_packet = AcknowledgeFinishConfigurationPacket;
                if let Some(command_sender) = &self.command_sender {
                    debug!(
                        "Sending acknowledge finish configuration packet: {:?}",
                        acknowledge_finish_configuration_packet
                    );
                    if let Err(e) = command_sender.try_send(
                        NetworkCommand::SendAcknowledgeFinishConfigurationPacket(
                            acknowledge_finish_configuration_packet,
                        ),
                    ) {
                        error!(
                            "Failed to send acknowledge finish configuration packet: {}",
                            e
                        );
                    }
                }
            }

            Event::KeepAliveConfiguration { packet } => {
                debug!("Received keep alive configuration packet: {:?}", packet);
                let keep_alive_packet = KeepAliveServerboundConfigurationPacket::new(packet.id);
                if let Some(command_sender) = &self.command_sender {
                    debug!(
                        "Sending keep alive configuration packet: {:?}",
                        keep_alive_packet
                    );
                    if let Err(e) = command_sender.try_send(
                        NetworkCommand::SendKeepAliveConfigurationPacket(keep_alive_packet),
                    ) {
                        error!("Failed to send keep alive configuration packet: {}", e);
                    }
                }
            }

            Event::KeepAlivePlay { packet } => {
                debug!("Received keep alive play packet: {:?}", packet);
                let keep_alive_packet = KeepAliveServerboundPlayPacket::new(packet.id);
                if let Some(command_sender) = &self.command_sender {
                    debug!("Sending keep alive play packet: {:?}", keep_alive_packet);
                    if let Err(e) = command_sender
                        .try_send(NetworkCommand::SendKeepAlivePlayPacket(keep_alive_packet))
                    {
                        error!("Failed to send keep alive play packet: {}", e);
                    }
                }
            }
            Event::Login { packet } => {
                debug!("Received login packet: {:?}", packet);
                info!(
                    "Joined: dimension: {} | game mode: {}",
                    packet.dimension_name, packet.game_mode
                );
            }
            Event::SynchronizePlayerPosition { packet } => {
                debug!("Received synchronize player position packet: {:?}", packet);
                let accept_teleportation_packet =
                    ConfirmTeleportationServerboundPacket::new(packet.teleport_id);
                if let Some(command_sender) = &self.command_sender {
                    debug!(
                        "Sending confirm teleportation packet: {:?}",
                        accept_teleportation_packet
                    );
                    if let Err(e) = command_sender.try_send(
                        NetworkCommand::SendConfirmTeleportationPacket(accept_teleportation_packet),
                    ) {
                        error!("Failed to send confirm teleportation packet: {}", e);
                    }
                }
            }
            Event::DisconnectLogin { packet } => {
                warn!("Received disconnect login packet: {:?}", packet.reason);
                info!("Disconnected during login");
                self.running = false;
            }
            Event::DisconnectConfiguration { packet } => {
                warn!(
                    "Received disconnect configuration packet: {:?}",
                    packet.reason
                );
                info!("Disconnected during configuration");
                self.running = false;
            }
            Event::DisconnectPlay { packet } => {
                warn!("Received disconnect play packet: {:?}", packet.reason);
                info!("Disconnected during play");
                self.running = false;
            }
            Event::SetCompression { .. } => {}
            _ => {
                debug!("Unhandled event: {:?}", event);
            }
        }
    }
}
