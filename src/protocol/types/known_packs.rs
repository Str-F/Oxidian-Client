use crate::protocol::types::mcstring::McString;

#[derive(Debug)]
pub struct KnownPacks {
    pub namespace: String,
    pub id: String,
    pub version: String,
}
