use crate::protocol::{state::ConnectionState, traits::packet::ServerboundPacket, varint};
use bytes::BytesMut;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ClientIntent {
    Status,
    Login,
    Transfer,
}

#[derive(Debug)]
pub struct HandshakePacket {
    protocol_version: i32,
    server_address: String,
    server_port: u16,
    intent: ClientIntent,
}

impl ClientIntent {
    pub fn id(&self) -> i32 {
        match self {
            ClientIntent::Status => 1,
            ClientIntent::Login => 2,
            ClientIntent::Transfer => 3,
        }
    }
}

impl HandshakePacket {
    pub fn new(
        protocol_version: i32,
        server_address: String,
        server_port: u16,
        intent: ClientIntent,
    ) -> Self {
        Self {
            protocol_version,
            server_address,
            server_port,
            intent,
        }
    }
}

impl ServerboundPacket for HandshakePacket {
    fn state(&self) -> ConnectionState {
        ConnectionState::Handshaking
    }

    fn id(&self) -> i32 {
        0
    }

    fn encode_data(&self) -> BytesMut {
        let mut buffer = BytesMut::new();
        varint::encode(&mut buffer, self.protocol_version);
        varint::encode(&mut buffer, self.server_address.as_bytes().len() as i32);
        buffer.extend_from_slice(self.server_address.as_bytes());
        buffer.extend_from_slice(&self.server_port.to_be_bytes());
        varint::encode(&mut buffer, self.intent.id());
        buffer
    }
}
