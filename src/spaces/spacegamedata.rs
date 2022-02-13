use super::spaces::SpaceName;
use crate::{
    players::player::PlayerName,
    units::{unitgamedata::UnitGameData, unitname::UnitName},
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SpaceGameData {
    pub owner_id: PlayerName,
    // TODO - logic for wall and fortress placement (since they can be placed independently of other placement and only one per turn)
    pub placements_this_turn: u64,
    pub conquered_this_turn: bool,
    pub units: HashMap<UnitName, HashMap<UnitGameData, u64>>,
}
impl Default for SpaceGameData {
    fn default() -> Self {
        Self {
            owner_id: PlayerName::Neutral,
            placements_this_turn: 0,
            conquered_this_turn: false,
            units: vec![].into_iter().collect(),
        }
    }
}
impl SpaceGameData {
    pub fn starting_value(space_name: &SpaceName) -> SpaceGameData {
        match space_name {
            SpaceName::NorthEredLuin => SpaceGameData {
                owner_id: PlayerName::Dwarves,
                ..Default::default()
            },
            SpaceName::EredLuin => SpaceGameData {
                owner_id: PlayerName::Dwarves,
                ..Default::default()
            },
            SpaceName::ThorinsHalls => SpaceGameData {
                owner_id: PlayerName::Dwarves,
                units: vec![
                    UnitName::DwarvenPikeman.tuple_with_default(1),
                    UnitName::Wall.tuple_with_default(7),
                ]
                .into_iter()
                .collect(),
                ..Default::default()
            },
            SpaceName::Lhun => SpaceGameData {
                ..Default::default()
            },
        }
    }
}
