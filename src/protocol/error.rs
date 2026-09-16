use crate::protocol::{packet::PacketError, types::mcstring::McStringError, varint::VarIntError};

#[derive(Debug)]
pub enum ProtocolError {
    VarInt(VarIntError),
    McString(McStringError),
    Packet(PacketError),
    Io(std::io::Error),
    InvalidData(String),
    UnexpectedEof,
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolError::VarInt(err) => write!(f, "VarInt error: {}", err),
            ProtocolError::McString(err) => write!(f, "McString error: {}", err),
            ProtocolError::Packet(err) => write!(f, "Packet error: {}", err),
            ProtocolError::Io(err) => write!(f, "IO error: {}", err),
            ProtocolError::InvalidData(err) => write!(f, "Invalid data: {}", err),
            ProtocolError::UnexpectedEof => write!(f, "Unexpected end of file"),
        }
    }
}

impl std::error::Error for ProtocolError {}

impl From<VarIntError> for ProtocolError {
    fn from(err: VarIntError) -> Self {
        ProtocolError::VarInt(err)
    }
}

impl From<McStringError> for ProtocolError {
    fn from(err: McStringError) -> Self {
        ProtocolError::McString(err)
    }
}

impl From<PacketError> for ProtocolError {
    fn from(err: PacketError) -> Self {
        ProtocolError::Packet(err)
    }
}

impl From<std::io::Error> for ProtocolError {
    fn from(err: std::io::Error) -> Self {
        ProtocolError::Io(err)
    }
}
