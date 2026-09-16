use crate::protocol::{error::ProtocolError, types::mcstring::McString};
use bytes::BytesMut;

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl Property {
    pub fn decode(data: &mut BytesMut) -> Result<Self, ProtocolError> {
        let name = McString::decode(data)?.0;
        let value = McString::decode(data)?.0;
        let has_signature = data
            .split_to(1)
            .first()
            .ok_or(ProtocolError::UnexpectedEof)?
            != &0;

        let signature = if has_signature {
            Some(McString::decode(data)?.0)
        } else {
            None
        };
        Ok(Property {
            name,
            value,
            signature,
        })
    }
}
