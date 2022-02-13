use crate::{player::Player, spaces::SpaceName, unitgamedata::UnitGameData, unitname::UnitName};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SpaceGameData {
    pub owner_id: Player,
    pub units: HashMap<UnitName, HashMap<UnitGameData, u64>>,
}
impl Default for SpaceGameData {
    fn default() -> Self {
        Self {
            owner_id: Player::Neutral,
            units: vec![].into_iter().collect(),
        }
    }
}
impl SpaceGameData {
    pub fn starting_value(space_name: &SpaceName) -> SpaceGameData {
        match space_name {
            SpaceName::NorthEredLuin => SpaceGameData {
                owner_id: Player::Dwarves,
                ..Default::default()
            },
            SpaceName::EredLuin => SpaceGameData {
                owner_id: Player::Dwarves,
                ..Default::default()
            },
            SpaceName::ThorinsHalls => SpaceGameData {
                owner_id: Player::Dwarves,
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
