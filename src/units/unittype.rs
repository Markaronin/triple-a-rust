use lazy_static::lazy_static;
use std::collections::HashMap;
use strum::IntoEnumIterator;

use super::unitname::UnitName;

pub enum TargetedAttack {}
pub enum Support {}
pub enum MovementType {
    Land { transporting_cost: u64 },
    Air { air_attack: u64, air_defense: u64 },
    Sea { transporting_capacity: u64 },
}
pub enum TerrainPreference {
    Open,
    Wilderness,
    Levy,
    Woodland,
    Dwarven,
    Ambusher,
    Relentless,
    Unyielding,
    Flying,
    Fortification,
    None,
    TODO,
}

pub struct UnitType {
    pub name: String,
    pub attack: u64,
    pub attack_rolls: u64,
    pub defense: u64,
    pub defense_rolls: u64,
    pub max_hp: u64,
    pub movement: u64,
    pub movement_type: MovementType,
    pub cost: u64,
    pub can_be_captured: bool,
    pub targeted_attacks: Vec<TargetedAttack>,
    pub support: Vec<Support>,
    pub terrain_preference: TerrainPreference,
}
impl Default for UnitType {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            attack: 0,
            attack_rolls: 1,
            defense: 0,
            defense_rolls: 1,
            max_hp: 1,
            movement: 0,
            movement_type: MovementType::Land {
                transporting_cost: 1,
            },
            cost: 0,
            can_be_captured: false,
            targeted_attacks: vec![],
            support: vec![],
            terrain_preference: TerrainPreference::TODO,
        }
    }
}

fn unit_types(unit_name: &UnitName) -> UnitType {
    match unit_name {
        UnitName::AncientTower => UnitType {
            name: "Ancient Tower".to_string(),
            defense: 4,
            defense_rolls: 3,
            can_be_captured: true,
            ..Default::default()
        },
        UnitName::AncientWall => UnitType {
            name: "Ancient Wall".to_string(),
            defense: 2,
            defense_rolls: 3,
            can_be_captured: true,
            ..Default::default()
        },
        UnitName::DwarvenAxeman => UnitType {
            name: "Dwarven Axeman".to_string(),
            attack: 5,
            defense: 3,
            movement: 2,
            cost: 7,
            ..Default::default()
        },
        UnitName::DwarvenAxethrower => UnitType {
            name: "Dwarven Axethrower".to_string(),
            attack: 2,
            defense: 2,
            movement: 2,
            cost: 5,
            ..Default::default()
        },
        UnitName::DwarvenHalberdier => UnitType {
            name: "Dwarven Halberdier".to_string(),
            attack: 6,
            defense: 4,
            movement: 2,
            cost: 8,
            ..Default::default()
        },
        UnitName::DwarvenPikeman => UnitType {
            name: "Dwarven Pikeman".to_string(),
            attack: 2,
            defense: 6,
            movement: 2,
            cost: 7,
            ..Default::default()
        },
        UnitName::Raven => UnitType {
            name: "Raven".to_string(),
            attack: 4,
            defense: 3,
            movement: 3,
            movement_type: MovementType::Air {
                air_attack: 1,
                air_defense: 1,
            },
            cost: 7,
            ..Default::default()
        },
        UnitName::Trebuchet => UnitType {
            name: "Trebuchet".to_string(),
            attack: 0,
            defense: 0,
            movement: 2,
            cost: 15,
            ..Default::default()
        },
        UnitName::Raft => UnitType {
            name: "Raft".to_string(),
            attack: 1,
            defense: 4,
            movement: 1,
            movement_type: MovementType::Sea {
                transporting_capacity: 2,
            },
            cost: 4,
            ..Default::default()
        },
        UnitName::Fortress => UnitType {
            name: "Fortress".to_string(),
            defense: 3,
            defense_rolls: 3,
            max_hp: 2,
            cost: 32,
            terrain_preference: TerrainPreference::None,
            ..Default::default()
        },
        UnitName::Wall => UnitType {
            name: "Wall".to_string(),
            defense: 3,
            max_hp: 2,
            cost: 8,
            terrain_preference: TerrainPreference::None,
            ..Default::default()
        },
    }
}

lazy_static! {
    pub static ref UNIT_TYPES: HashMap<UnitName, UnitType> = UnitName::iter()
        .map(|unit_name| (unit_name.clone(), unit_types(&unit_name)))
        .collect();
}
