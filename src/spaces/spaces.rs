use crate::terrain::Terrain;
use lazy_static::lazy_static;
use maplit::hashset;
use std::collections::{HashMap, HashSet};
use strum::EnumIter;
use strum::EnumString;
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq, Eq, Hash, EnumIter, EnumString, Clone)]
pub enum SpaceName {
    NorthEredLuin,
    ThorinsHalls,
    EredLuin,
    LittleLhun,
    Lhun,
    NorthForlindon,
    RiverOfForlindon,
    BlueMountains,
    LhunWestBank,
    SouthForlindon,
    Forlond,
    NorthLindon,
    Mithlond,
}

pub struct Space {
    pub name: String,
    pub connections: HashSet<SpaceName>,
    pub money_production: u64,
    pub unit_production: u64,
    pub terrain: HashSet<Terrain>,
}
impl Default for Space {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            connections: HashSet::new(),
            money_production: 0,
            unit_production: 0,
            terrain: HashSet::new(),
        }
    }
}

fn spaces(space_name: &SpaceName) -> Space {
    match space_name {
        SpaceName::NorthEredLuin => Space {
            name: "North Ered Luin".to_string(),
            connections: hashset! {SpaceName::ThorinsHalls, SpaceName::EredLuin, SpaceName::LittleLhun, SpaceName::Lhun},
            money_production: 1,
            terrain: hashset! {Terrain::Mountain},
            ..Default::default()
        },
        SpaceName::EredLuin => Space {
            name: "Ered Luin".to_string(),
            connections: hashset! {SpaceName::ThorinsHalls, SpaceName::NorthEredLuin, SpaceName::LittleLhun, SpaceName::BlueMountains, SpaceName::RiverOfForlindon, SpaceName::NorthForlindon},
            money_production: 1,
            terrain: hashset! {Terrain::Mountain},
            ..Default::default()
        },
        SpaceName::ThorinsHalls => Space {
            name: "Thorin's Halls".to_string(),
            connections: hashset! {SpaceName::NorthEredLuin, SpaceName::EredLuin, SpaceName::LittleLhun},
            money_production: 3,
            unit_production: 5,
            terrain: hashset! {Terrain::Mountain, Terrain::Settlement},
            ..Default::default()
        },
        SpaceName::Lhun => Space {
            name: "Lhun".to_string(),
            connections: hashset! {SpaceName::NorthEredLuin, SpaceName::LittleLhun},
            money_production: 1,
            terrain: hashset! {Terrain::River},
            ..Default::default()
        },
        SpaceName::LittleLhun => Space {
            name: "Little Lhun".to_string(),
            connections: hashset! {SpaceName::Lhun, SpaceName::NorthEredLuin, SpaceName::ThorinsHalls, SpaceName::EredLuin, SpaceName::BlueMountains, SpaceName::LhunWestBank},
            money_production: 1,
            terrain: hashset! {Terrain::River},
            ..Default::default()
        },
        SpaceName::NorthForlindon => Space {
            name: "North Forlindon".to_string(),
            connections: hashset! {SpaceName::EredLuin, SpaceName::RiverOfForlindon, SpaceName::SouthForlindon},
            money_production: 1,
            terrain: hashset! {Terrain::Plains},
            ..Default::default()
        },
        SpaceName::RiverOfForlindon => Space {
            name: "River of Forlindon".to_string(),
            connections: hashset! {SpaceName::NorthForlindon, SpaceName::EredLuin, SpaceName::BlueMountains, SpaceName::NorthLindon, SpaceName::Forlond, SpaceName::SouthForlindon},
            money_production: 1,
            terrain: hashset! {Terrain::River},
            ..Default::default()
        },
        SpaceName::BlueMountains => Space {
            name: "Blue Mountains".to_string(),
            connections: hashset! {SpaceName::EredLuin, SpaceName::LittleLhun, SpaceName::LhunWestBank, SpaceName::NorthLindon, SpaceName::RiverOfForlindon},
            money_production: 2,
            terrain: hashset! {Terrain::Mountain, Terrain::Forest},
            ..Default::default()
        },
        SpaceName::LhunWestBank => Space {
            name: "Lhun West Bank".to_string(),
            connections: hashset! {SpaceName::LittleLhun, SpaceName::BlueMountains, SpaceName::NorthLindon, SpaceName::Mithlond},
            money_production: 1,
            terrain: hashset! {Terrain::Plains},
            ..Default::default()
        },
        SpaceName::SouthForlindon => Space {
            name: "South Forlindon".to_string(),
            connections: hashset! {SpaceName::NorthForlindon, SpaceName::RiverOfForlindon, SpaceName::Forlond},
            money_production: 1,
            terrain: hashset! {Terrain::Plains},
            ..Default::default()
        },
        SpaceName::Forlond => Space {
            name: "Forlond".to_string(),
            connections: hashset! {SpaceName::SouthForlindon, SpaceName::RiverOfForlindon, SpaceName::NorthLindon},
            money_production: 2,
            unit_production: 3,
            terrain: hashset! {Terrain::Settlement},
            ..Default::default()
        },
        SpaceName::NorthLindon => Space {
            name: "North Lindon".to_string(),
            connections: hashset! {SpaceName::Forlond, SpaceName::RiverOfForlindon, SpaceName::BlueMountains, SpaceName::LhunWestBank, SpaceName::Mithlond},
            money_production: 1,
            terrain: hashset! {Terrain::Plains},
            ..Default::default()
        },
        SpaceName::Mithlond => Space {
            name: "Mithlond".to_string(),
            connections: hashset! {SpaceName::NorthLindon, SpaceName::LhunWestBank},
            money_production: 4,
            unit_production: 5,
            terrain: hashset! {Terrain::Settlement, Terrain::River},
            ..Default::default()
        },
    }
}

lazy_static! {
    pub static ref SPACES: HashMap<SpaceName, Space> = SpaceName::iter()
        .map(|space_name| (space_name.clone(), spaces(&space_name)))
        .collect();
}
