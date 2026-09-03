use crate::client;
use crate::network::command::NetworkCommand;
use crate::network::connection::Connection;
use crate::protocol::dispatcher::{Event, PacketDispatcher};
use crate::protocol::packets::status::status_request::StatusRequestPacket;
use crate::protocol::packets::{
    handshake::{ClientIntent, HandshakePacket},
    login::login_start::LoginStartPacket,
};
use crate::protocol::state::ConnectionState;
use crate::server::Server;

use std::io::Error;
use tokio::sync::mpsc;
use uuid::Uuid;

pub struct NetworkManager {
    connection: Connection,
    dispatcher: PacketDispatcher,
}

impl NetworkManager {
    pub async fn get_status_server(
        server: &Server,
    ) -> Result<(mpsc::Sender<NetworkCommand>, mpsc::Receiver<Event>), Error> {
        let mut connection = Connection::new(server.host(), server.port()).await?;

        let handshake = HandshakePacket::new(
            client::PROTOCOL_VER,
            server.host().to_string(),
            server.port(),
            ClientIntent::Status,
        );

        println!("Sending handshake packet to server: {:?}", handshake);

        connection.send(handshake).await?;

        connection.set_state(ConnectionState::Status);
        println!("Changed connection state to Status");

        let status_request_packet = StatusRequestPacket::new();

        println!(
            "Sending status request packet to server: {:?}",
            status_request_packet
        );

        connection.send(status_request_packet).await?;

        let (command_sender, command_receiver) = mpsc::channel::<NetworkCommand>(100);
        let (event_sender, event_receiver) = mpsc::channel::<Event>(100);

        let manager = Self {
            connection,
            dispatcher: PacketDispatcher::new(),
        };

        tokio::spawn(async move {
            manager.run(command_receiver, event_sender).await;
        });

        Ok((command_sender, event_receiver))
    }

    pub async fn join_server(
        server: &Server,
    ) -> Result<(mpsc::Sender<NetworkCommand>, mpsc::Receiver<Event>), Error> {
        let mut connection = Connection::new(server.host(), server.port()).await?;

        let handshake = HandshakePacket::new(
            client::PROTOCOL_VER,
            server.host().to_string(),
            server.port(),
            ClientIntent::Login,
        );

        println!("Sending handshake packet to server: {:?}", handshake);

        connection.send(handshake).await?;

        connection.set_state(ConnectionState::Login);
        println!("Changed connection state to Login");

        let login_start_packet =
            LoginStartPacket::new(client::USERNAME.to_string(), Uuid::new_v4());

        println!(
            "Sending login start packet to server: {:?}",
            login_start_packet
        );

        connection.send(login_start_packet).await?;

        let (command_sender, command_receiver) = mpsc::channel::<NetworkCommand>(100);
        let (event_sender, event_receiver) = mpsc::channel::<Event>(100);

        let manager = Self {
            connection,
            dispatcher: PacketDispatcher::new(),
        };

        tokio::spawn(async move {
            manager.run(command_receiver, event_sender).await;
        });

        Ok((command_sender, event_receiver))
    }

    pub async fn run(
        mut self,
        mut command_receiver: mpsc::Receiver<NetworkCommand>,
        event_sender: mpsc::Sender<Event>,
    ) {
        loop {
            tokio::select! {
                Some(command) = command_receiver.recv() => {
                    self.handle_command(command).await;
                }
                result = self.connection.read_packet() => {
                    match result {
                        Ok((packet_id, mut packet_data)) => {
                            match self.dispatcher.dispatch(
                                self.connection.state(),
                                packet_id,
                                &mut packet_data,
                            ) {
                                Ok(event) => {
                                    if let Err(e) = event_sender.send(event).await {
                                        eprintln!("Failed to send event: {}", e);
                                        break;
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to dispatch packet: {} , Error: {:?}", packet_id, e);
                                }
                            }
                        }
                        Err(e) => {
                            if self.connection.state() == ConnectionState::Status {
                                println!("Server closed connection as expected (Status complete).");
                            } else {
                                eprintln!("Connection closed unexpectedly: {}", e);
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    async fn handle_command(&mut self, command: NetworkCommand) {
        match command {
            NetworkCommand::SendPingRequestPacket(ping_request_packet) => {
                if let Err(e) = self.connection.send(ping_request_packet).await {
                    eprintln!("Failed to send ping request packet: {}", e);
                    return;
                }
            }

            NetworkCommand::SendLoginAcknowledgedPacket(login_ack_packet) => {
                if let Err(e) = self.connection.send(login_ack_packet).await {
                    eprintln!("Failed to send login acknowledged packet: {}", e);
                    return;
                }
                println!("Changing State to Configuration");
                self.connection.set_state(ConnectionState::Configuration);
            }
            NetworkCommand::SendKnownPacksPacket(known_packs_packet) => {
                if let Err(e) = self.connection.send(known_packs_packet).await {
                    eprintln!("Failed to send known packs packet: {}", e);
                    return;
                }
            }
            NetworkCommand::SendAcknowledgeFinishConfigurationPacket(
                acknowledge_finish_configuration_packet,
            ) => {
                if let Err(e) = self
                    .connection
                    .send(acknowledge_finish_configuration_packet)
                    .await
                {
                    eprintln!(
                        "Failed to send acknowledge finish configuration packet: {}",
                        e
                    );
                    return;
                }
                println!("Changing State to Play");
                self.connection.set_state(ConnectionState::Play);
            }
        }
    }
}
