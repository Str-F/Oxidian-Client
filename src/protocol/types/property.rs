use crate::protocol::types::mcstring::McString;
use bytes::BytesMut;
use std::io::ErrorKind;
use std::io::{Error, ErrorKind::UnexpectedEof};

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl Property {
    pub fn decode(data: &mut BytesMut) -> Result<Self, Error> {
        if data.len() < 16 {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Not enough data to read UUID",
            ));
        }

        let name = McString::decode(data)?.0;
        let value = McString::decode(data)?.0;
        let has_signature = data
            .split_to(1)
            .first()
            .ok_or(Error::new(UnexpectedEof, "Missing signature flag"))?
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
