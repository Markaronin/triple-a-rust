use crate::terrain::Terrain;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use strum::EnumIter;
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq, Eq, Hash, EnumIter, Clone)]
pub enum SpaceName {
    NorthEredLuin,
    ThorinsHalls,
    EredLuin,
    Lhun,
}

pub struct Space {
    pub name: String,
    pub connections: Vec<SpaceName>,
    pub money_production: u64,
    pub unit_production: u64,
    pub terrain: HashSet<Terrain>,
}
impl Default for Space {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            connections: vec![],
            money_production: 0,
            unit_production: 0,
            terrain: vec![].into_iter().collect(),
        }
    }
}

fn spaces(space_name: &SpaceName) -> Space {
    match space_name {
        SpaceName::NorthEredLuin => Space {
            name: "North Ered Luin".to_string(),
            connections: vec![SpaceName::ThorinsHalls, SpaceName::EredLuin],
            money_production: 1,
            terrain: vec![Terrain::Mountain].into_iter().collect(),
            ..Default::default()
        },
        SpaceName::EredLuin => Space {
            name: "Ered Luin".to_string(),
            connections: vec![SpaceName::ThorinsHalls, SpaceName::NorthEredLuin],
            money_production: 1,
            terrain: vec![Terrain::Mountain].into_iter().collect(),
            ..Default::default()
        },
        SpaceName::ThorinsHalls => Space {
            name: "Thorin's Halls".to_string(),
            connections: vec![SpaceName::NorthEredLuin, SpaceName::EredLuin],
            money_production: 3,
            unit_production: 5,
            terrain: vec![Terrain::Mountain, Terrain::Settlement]
                .into_iter()
                .collect(),
            ..Default::default()
        },
        SpaceName::Lhun => Space {
            name: "Lhun".to_string(),
            connections: vec![SpaceName::NorthEredLuin],
            money_production: 1,
            terrain: vec![Terrain::Plains].into_iter().collect(),
            ..Default::default()
        },
    }
}

lazy_static! {
    pub static ref SPACES: HashMap<SpaceName, Space> = SpaceName::iter()
        .map(|space_name| (space_name.clone(), spaces(&space_name)))
        .collect();
}
