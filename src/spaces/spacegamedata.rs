use super::spaces::SpaceName;
use crate::{
    players::player::PlayerName,
    units::{unitgamedata::UnitGameData, unitname::UnitName},
};
use druid::Data;
use std::collections::HashMap;

#[derive(Clone, Debug, Data, PartialEq)]
pub struct SpaceGameData {
    pub owner_id: PlayerName,
    // TODO - logic for wall and fortress placement (since they can be placed independently of other placement and only one per turn)
    pub placements_this_turn: usize,
    pub conquered_this_turn: bool,
    #[data(eq)]
    pub units: HashMap<UnitName, HashMap<UnitGameData, usize>>,
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
            SpaceName::EredLuin => SpaceGameData {
                owner_id: PlayerName::Dwarves,
                ..Default::default()
            },
            SpaceName::LittleLhun => SpaceGameData {
                ..Default::default()
            },
            SpaceName::Lhun => SpaceGameData {
                ..Default::default()
            },
            SpaceName::NorthForlindon => SpaceGameData {
                owner_id: PlayerName::HighElves,
                ..Default::default()
            },
            SpaceName::RiverOfForlindon => SpaceGameData {
                owner_id: PlayerName::HighElves,
                ..Default::default()
            },
            SpaceName::BlueMountains => SpaceGameData {
                owner_id: PlayerName::Dwarves,
                units: vec![UnitName::DwarvenAxethrower.tuple_with_default(1)]
                    .into_iter()
                    .collect(),
                ..Default::default()
            },
            SpaceName::LhunWestBank => SpaceGameData {
                owner_id: PlayerName::HighElves,
                ..Default::default()
            },
            SpaceName::SouthForlindon => SpaceGameData {
                owner_id: PlayerName::HighElves,
                ..Default::default()
            },
            SpaceName::Forlond => SpaceGameData {
                owner_id: PlayerName::HighElves,
                units: vec![UnitName::NoldorinWarrior.tuple_with_default(1)]
                    .into_iter()
                    .collect(),
                ..Default::default()
            },
            SpaceName::NorthLindon => SpaceGameData {
                owner_id: PlayerName::HighElves,
                units: vec![UnitName::ElvenCavalry.tuple_with_default(2)]
                    .into_iter()
                    .collect(),
                ..Default::default()
            },
            SpaceName::Mithlond => SpaceGameData {
                owner_id: PlayerName::HighElves,
                units: vec![
                    UnitName::NoldorinWarrior.tuple_with_default(1),
                    UnitName::ElvenArcher.tuple_with_default(1),
                    UnitName::Wall.tuple_with_default(1),
                ]
                .into_iter()
                .collect(),
                ..Default::default()
            },
        }
    }
}
