use crate::protocol::{packet, state::ConnectionState, traits::packet::ServerboundPacket};
use bytes::BytesMut;
use std::io::Error;
use std::io::ErrorKind;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
    state: ConnectionState,
    buffer: BytesMut,
    compression_threshold: Option<i32>,
}

impl Connection {
    pub async fn new(address: &str, port: u16) -> Result<Self, Error> {
        let stream = TcpStream::connect(format!("{}:{}", address, port)).await?;
        Ok(Connection {
            stream,
            state: ConnectionState::Handshaking,
            buffer: BytesMut::new(),
            compression_threshold: None,
        })
    }

    pub fn set_state(&mut self, state: ConnectionState) {
        self.state = state;
    }

    pub fn state(&self) -> ConnectionState {
        self.state
    }

    pub fn set_compression_threshold(&mut self, threshold: i32) {
        if threshold < 0 {
            self.compression_threshold = None;
            println!("Disabling compression");
        } else {
            self.compression_threshold = Some(threshold);
            println!("Setting compression threshold to: {}", threshold);
        }
    }

    pub async fn send<P: ServerboundPacket>(&mut self, packet_data: P) -> Result<(), Error> {
        if self.state != P::state() {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Cannot send packet in state {:?}, expected state {:?}",
                    self.state,
                    P::state()
                ),
            ));
        }
        let data = packet::encode(
            P::id(),
            &packet_data.encode_data(),
            self.compression_threshold,
        )?;
        println!("Sending packet id: {}", P::id());
        println!("{:02X?}", data);
        self.stream.write_all(&data).await?;
        Ok(())
    }

    pub async fn read_from_stream(&mut self) -> Result<(), Error> {
        let mut temp_buffer = [0u8; 8192];
        let bytes_read = self.stream.read(&mut temp_buffer).await?;
        if bytes_read == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Connection closed by server",
            ));
        }
        self.buffer.extend_from_slice(&temp_buffer[..bytes_read]);
        Ok(())
    }

    pub async fn read_packet(&mut self) -> Result<(i32, BytesMut), Error> {
        loop {
            if let Some(packet) = packet::decode(&mut self.buffer, self.compression_threshold)? {
                return Ok(packet);
            }

            self.read_from_stream().await?;
        }
    }
}
