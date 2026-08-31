use crate::network::command::NetworkCommand;
use crate::network::manager::NetworkManager;
use crate::protocol::dispatcher::Event;
use crate::protocol::packets::configuration::serverbound::acknowledge_finish_configuration::AcknowledgeFinishConfigurationPacket;
use crate::protocol::packets::configuration::serverbound::known_packs::KnownPacksServerboundPacket;
use crate::protocol::packets::login::login_acknowledged::LoginAcknowledgedPacket;
use crate::server::Server;
use tokio::sync::mpsc;

pub const PROTOCOL_VER: i32 = 776;
pub const MC_VER: &str = "26.2";
pub const USERNAME: &str = "OxidianClientUsr";

pub struct Client {
    event_receiver: Option<mpsc::Receiver<Event>>,
    command_sender: Option<mpsc::Sender<NetworkCommand>>,
    running: bool,
}

impl Client {
    pub fn new() -> Self {
        Self {
            event_receiver: None,
            command_sender: None,
            running: true,
        }
    }

    pub async fn run(&mut self) {
        self.connect_server().await;

        while self.running {
            self.update();
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }

    pub async fn connect_server(&mut self) {
        let server = Server::new("localhost", 25565);

        match NetworkManager::join_server(&server).await {
            Ok((command_sender, event_receiver)) => {
                self.command_sender = Some(command_sender);
                self.event_receiver = Some(event_receiver);
                println!("Connected to server: {}:{}", server.host(), server.port());
            }
            Err(e) => {
                eprintln!(
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
                        eprintln!("Event channel disconnected");
                        self.running = false;
                        break;
                    }
                }
            }
        }

        for event in events {
            println!("Received event: {:?}", event);
            self.handle_event(event);
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::LoginSuccess { packet } => {
                println!("Handling packet: {:?}", packet);
                println!(
                    "Logged in as: UUID: {}, Username: {}",
                    packet.profile.uuid, packet.profile.name
                );
                println!("Session ID: {}", packet.session_id);
                println!("Profile Properties:");
                for property in packet.profile.properties {
                    println!(
                        "  Name: {}, Value: {}, Signature: {:?}",
                        property.name, property.value, property.signature
                    );
                }
                let login_ack_packet = LoginAcknowledgedPacket::new();
                if let Some(command_sender) = &self.command_sender {
                    println!("Sending login acknowledged packet: {:?}", login_ack_packet);
                    if let Err(e) = command_sender.try_send(
                        NetworkCommand::SendLoginAcknowledgedPacket(login_ack_packet),
                    ) {
                        eprintln!("Failed to send login acknowledged packet: {}", e);
                    }
                }
            }

            Event::LoginDisconnect => {
                println!("Received login disconnect packet");
                println!("Disconnected during login");
            }

            Event::KnownPacks { packet } => {
                println!("Received known packs packet: {:?}", packet);
                let known_packs_packet = KnownPacksServerboundPacket::new(packet.packs);
                if let Some(command_sender) = &self.command_sender {
                    println!("Sending known packs packet: {:?}", known_packs_packet);
                    if let Err(e) = command_sender
                        .try_send(NetworkCommand::SendKnownPacksPacket(known_packs_packet))
                    {
                        eprintln!("Failed to send known packs packet: {}", e);
                    }
                }
            }

            Event::FinishConfiguration { packet } => {
                println!("Received finish configuration packet: {:?}", packet);
                let acknowledge_finish_configuration_packet = AcknowledgeFinishConfigurationPacket;
                if let Some(command_sender) = &self.command_sender {
                    println!(
                        "Sending acknowledge finish configuration packet: {:?}",
                        acknowledge_finish_configuration_packet
                    );
                    if let Err(e) = command_sender.try_send(
                        NetworkCommand::SendAcknowledgeFinishConfigurationPacket(
                            acknowledge_finish_configuration_packet,
                        ),
                    ) {
                        eprintln!(
                            "Failed to send acknowledge finish configuration packet: {}",
                            e
                        );
                    }
                }
            }
            _ => {
                println!("Unhandled event: {:?}", event);
            }
        }
    }
}
