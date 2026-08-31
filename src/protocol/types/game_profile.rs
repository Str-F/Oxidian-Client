use bytes::BytesMut;
use std::io::{Error, ErrorKind};

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
    pub fn decode(data: &mut BytesMut) -> Result<Self, Error> {
        if data.len() < 16 {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Not enough data to read UUID",
            ));
        }

        let uuid = uuid::Uuid::from_slice(&data.split_to(16))
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid UUID"))?;

        let name = McString::decode(data)?.0;

        let properties_length = varint::decode(data)
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid property length"))?
            .0 as usize;

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
