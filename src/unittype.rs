use crate::unitname::UnitName;
use lazy_static::lazy_static;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub enum TargetedAttack {}
pub enum Support {}
pub enum MovementType {
    Land,
    Air,
    Sea,
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
    pub defense: u64,
    pub max_hp: u64,
    pub movement: u64,
    pub movement_type: MovementType,
    pub cost: u64,
    pub targeted_attacks: Vec<TargetedAttack>,
    pub support: Vec<Support>,
    pub terrain_preference: TerrainPreference,
}
impl Default for UnitType {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            attack: 0,
            defense: 0,
            max_hp: 1,
            movement: 0,
            movement_type: MovementType::Land,
            cost: 0,
            targeted_attacks: vec![],
            support: vec![],
            terrain_preference: TerrainPreference::TODO,
        }
    }
}

fn unit_types(unit_name: &UnitName) -> UnitType {
    match unit_name {
        UnitName::DwarvenPikeman => UnitType {
            name: "Dwarven Pikeman".to_string(),
            attack: 2,
            defense: 6,
            movement: 2,
            cost: 7,
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
