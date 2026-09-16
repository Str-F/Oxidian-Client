use bytes::BytesMut;

use crate::protocol::error::ProtocolError;
use crate::protocol::types::mcstring::McString;
use crate::protocol::types::property::Property;
use crate::protocol::varint;

#[derive(Debug)]
pub struct GameProfile {
    pub uuid: uuid::Uuid,
    pub name: String,
    pub properties: Vec<Property>,
}

impl GameProfile {
    pub fn decode(data: &mut BytesMut) -> Result<Self, ProtocolError> {
        if data.len() < 16 {
            return Err(ProtocolError::UnexpectedEof);
        }

        let uuid = uuid::Uuid::from_slice(&data.split_to(16))
            .map_err(|_| ProtocolError::InvalidData("Invalid UUID".to_string()))?;

        let name = McString::decode(data)?.0;

        let properties_length = varint::decode(data)?.0 as usize;

        let mut properties = Vec::new();

        for _ in 0..properties_length {
            properties.push(Property::decode(data)?);
        }

        Ok(Self {
            uuid,
            name,
            properties,
        })
    }
}
