use crate::protocol::varint;
use bytes::Buf;
use bytes::BytesMut;
use fastnbt::DeOpts;

use crate::protocol::state::ConnectionState;
use crate::protocol::types::mcstring::McString;
use crate::protocol::{traits::packet::ClientboundPacket, types::registry_entry::RegistryEntry};

#[derive(Debug)]
pub struct RegistryDataClientboundPacket {
    pub id: String,
    pub entries: Vec<RegistryEntry>,
}

impl RegistryDataClientboundPacket {
    pub fn decode(read: &mut BytesMut) -> Option<Self> {
        let registry_id = McString::decode(read).ok()?.0;

        let count = varint::decode_mut(read).ok()? as usize;
        let mut entries = Vec::with_capacity(count);

        for _ in 0..count {
            let entry_id = McString::decode(read).ok()?.0;

            if !read.has_remaining() {
                return None;
            }
            let has_data = read.get_u8() != 0;

            let data = if has_data {
                let mut slice = &read[..];
                let len_before = slice.len();

                let value =
                    fastnbt::from_reader_with_opts(&mut slice, DeOpts::network_nbt()).ok()?;

                let bytes_read = len_before - slice.len();
                read.advance(bytes_read);

                Some(value)
            } else {
                None
            };

            entries.push(RegistryEntry { id: entry_id, data });
        }
        Some(Self {
            id: registry_id,
            entries,
        })
    }
}

impl ClientboundPacket for RegistryDataClientboundPacket {
    fn state(&self) -> ConnectionState {
        ConnectionState::Configuration
    }

    fn id(&self) -> i32 {
        7
    }
}
