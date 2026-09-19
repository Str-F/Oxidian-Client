use crate::protocol::{
    error::ProtocolError,
    state::ConnectionState,
    traits::packet::ClientboundPacket,
    types::{mcstring::McString, position::Position},
    varint,
};
use bytes::Buf;
use bytes::BytesMut;

#[derive(Debug)]
pub struct LoginClientboundPacket {
    entity_id: i32,
    is_hardcore: bool,
    dimension_names: Vec<String>,
    max_players: i32,
    view_distance: i32,
    simulation_distance: i32,
    reduced_debug_info: bool,
    enable_respawn_screen: bool,
    do_limited_crafting: bool,
    dimension_type: i32,
    dimension_name: String,
    hashed_seed: i64,
    game_mode: u8,
    previous_game_mode: i8,
    is_debug: bool,
    is_flat: bool,
    // has_death_location: bool,
    death_dimension_name: Option<String>,
    death_location: Option<Position>,
    portal_cooldown: i32,
    sea_level: i32,
    online_mode: bool,
    enforce_secure_chat: bool,
}

impl LoginClientboundPacket {
    pub fn decode(data: &mut BytesMut) -> Result<Self, ProtocolError> {
        let entity_id = data.get_i32();
        let is_hardcore = data.get_u8() != 0;
        let dimension_count = varint::decode_mut(data)? as usize;
        let mut dimension_names = Vec::with_capacity(dimension_count);
        for _ in 0..dimension_count {
            let dimension_name = McString::decode(data)?.0;
            dimension_names.push(dimension_name);
        }
        let max_players = varint::decode_mut(data)?;
        let view_distance = varint::decode_mut(data)?;
        let simulation_distance = varint::decode_mut(data)?;
        let reduced_debug_info = data.get_u8() != 0;
        let enable_respawn_screen = data.get_u8() != 0;
        let do_limited_crafting = data.get_u8() != 0;
        let dimension_type = varint::decode_mut(data)?;
        let dimension_name = McString::decode(data)?.0;
        let hashed_seed = data.get_i64();
        let game_mode = data.get_u8();
        let previous_game_mode = data.get_i8();
        let is_debug = data.get_u8() != 0;
        let is_flat = data.get_u8() != 0;
        let (death_dimension_name, death_location) = if data.get_u8() != 0 {
            let death_dimension_name = McString::decode(data)?.0;
            let death_location = Position::decode(data)?;
            (Some(death_dimension_name), Some(death_location))
        } else {
            (None, None)
        };
        let portal_cooldown = varint::decode_mut(data)?;
        let sea_level = varint::decode_mut(data)?;
        let online_mode = data.get_u8() != 0;
        let enforce_secure_chat = data.get_u8() != 0;

        Ok(Self {
            entity_id,
            is_hardcore,
            dimension_names,
            max_players,
            view_distance,
            simulation_distance,
            reduced_debug_info,
            enable_respawn_screen,
            do_limited_crafting,
            dimension_type,
            dimension_name,
            hashed_seed,
            game_mode,
            previous_game_mode,
            is_debug,
            is_flat,
            death_dimension_name,
            death_location,
            portal_cooldown,
            sea_level,
            online_mode,
            enforce_secure_chat,
        })
    }
}

impl ClientboundPacket for LoginClientboundPacket {
    fn state() -> crate::protocol::state::ConnectionState {
        ConnectionState::Play
    }

    fn id() -> i32 {
        49
    }
}
