use crate::spacegamedata::SpaceGameData;
use crate::terrain::Terrain;
use crate::{player::Player, unitname::UnitName};
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use strum::EnumIter;
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq, Eq, Hash, EnumIter, Clone)]
pub enum SpaceName {
    NorthEredLuin,
    ThorinsHalls,
}

pub struct Space {
    pub name: String,
    pub connections: Vec<SpaceName>,
    pub terrain: HashSet<Terrain>,
    pub starting_value: SpaceGameData,
}
impl Default for Space {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            connections: vec![],
            terrain: vec![].into_iter().collect(),
            starting_value: Default::default(),
        }
    }
}

fn spaces(space_name: &SpaceName) -> Space {
    match space_name {
        SpaceName::NorthEredLuin => Space {
            name: "North Ered Luin".to_string(),
            connections: vec![SpaceName::ThorinsHalls],
            terrain: vec![Terrain::Mountain].into_iter().collect(),
            starting_value: SpaceGameData {
                owner_id: Player::Dwarves,
                units: vec![].into_iter().collect(),
            },
            ..Default::default()
        },
        SpaceName::ThorinsHalls => Space {
            name: "Thorin's Halls".to_string(),
            connections: vec![SpaceName::NorthEredLuin],
            terrain: vec![Terrain::Mountain, Terrain::Settlement]
                .into_iter()
                .collect(),
            starting_value: SpaceGameData {
                owner_id: Player::Dwarves,
                units: vec![
                    UnitName::DwarvenPikeman.tuple_with_default(1),
                    UnitName::Wall.tuple_with_default(7),
                ]
                .into_iter()
                .collect(),
            },
            ..Default::default()
        },
    }
}

lazy_static! {
    pub static ref SPACES: HashMap<SpaceName, Space> = SpaceName::iter()
        .map(|space_name| (space_name.clone(), spaces(&space_name)))
        .collect();
}
