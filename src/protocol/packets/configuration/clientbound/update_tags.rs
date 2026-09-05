use crate::protocol::types::mcstring::McString;
use crate::protocol::varint;
use crate::protocol::{
    state::ConnectionState, traits::packet::ClientboundPacket, types::tags_field::TagsField,
};
use bytes::BytesMut;

#[derive(Debug)]
pub struct RegistryTags {
    pub registry: String,
    pub tags: Vec<TagsField>,
}

#[derive(Debug)]
pub struct UpdateTagsClientboundPacket {
    pub registries: Vec<RegistryTags>,
}

impl UpdateTagsClientboundPacket {
    pub fn decode(read: &mut BytesMut) -> Option<Self> {
        let registry_count = varint::decode_mut(read).ok()? as usize;
        let mut registries = Vec::with_capacity(registry_count);

        for _ in 0..registry_count {
            let registry_id = McString::decode(read).ok()?.0;

            let tag_count = varint::decode_mut(read).ok()? as usize;
            let mut tags = Vec::with_capacity(tag_count);

            for _ in 0..tag_count {
                let tag_name = McString::decode(read).ok()?.0;

                let entry_count = varint::decode_mut(read).ok()? as usize;
                let mut entries = Vec::with_capacity(entry_count);

                for _ in 0..entry_count {
                    entries.push(varint::decode_mut(read).ok()?);
                }

                tags.push(TagsField {
                    name: tag_name,
                    entries,
                });
            }

            registries.push(RegistryTags {
                registry: registry_id,
                tags,
            });
        }

        Some(Self { registries })
    }
}

impl ClientboundPacket for UpdateTagsClientboundPacket {
    fn state(&self) -> crate::protocol::state::ConnectionState {
        ConnectionState::Configuration
    }

    fn id(&self) -> i32 {
        13
    }
}
