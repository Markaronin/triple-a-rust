use super::{
    support_types::SupportType, targeted_attacks::TargetedAttack,
    terrain_preference::TerrainPreference, unit_type::UnitType, unitname::UnitName,
};
use lazy_static::lazy_static;
use maplit::hashset;
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

pub enum MovementType {
    Land {
        transporting_cost: usize,
    },
    Air {
        air_attack: usize,
        air_defense: usize,
    },
    Sea {
        transporting_capacity: usize,
    },
}

pub struct Unit {
    pub name: String,
    pub attack: usize,
    pub attack_rolls: usize,
    pub defense: usize,
    pub defense_rolls: usize,
    pub max_hp: usize,
    pub movement: usize,
    pub movement_type: MovementType,
    pub cost: usize,
    pub can_be_captured: bool,
    pub targeted_attacks: HashSet<TargetedAttack>,
    pub unit_type: HashSet<UnitType>,
    pub support: HashSet<SupportType>,
    pub terrain_preference: Option<TerrainPreference>,
}
impl Default for Unit {
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
            targeted_attacks: HashSet::new(),
            unit_type: HashSet::new(),
            support: HashSet::new(),
            terrain_preference: None,
        }
    }
}

fn units(unit_name: &UnitName) -> Unit {
    match unit_name {
        UnitName::AncientTower => Unit {
            name: "Ancient Tower".to_string(),
            defense: 4,
            defense_rolls: 3,
            can_be_captured: true,
            unit_type: hashset! {UnitType::Fortification, UnitType::Ancient},
            support: hashset! {SupportType::Battlements{ amount: 2, units: 6}},
            terrain_preference: Some(TerrainPreference::Fortification),
            ..Default::default()
        },
        UnitName::AncientWall => Unit {
            name: "Ancient Wall".to_string(),
            defense: 2,
            defense_rolls: 3,
            can_be_captured: true,
            unit_type: hashset! {UnitType::Fortification, UnitType::Ancient},
            support: hashset! {SupportType::Battlements{ amount: 1, units: 6}},
            terrain_preference: Some(TerrainPreference::Fortification),
            ..Default::default()
        },
        UnitName::DwarvenAxeman => Unit {
            name: "Dwarven Axeman".to_string(),
            attack: 5,
            defense: 3,
            movement: 2,
            cost: 7,
            unit_type: hashset! {UnitType::Melee, UnitType::Infantry},
            support: hashset! {SupportType::Shield{ amount: 3, units: 1}},
            terrain_preference: Some(TerrainPreference::Dwarven),
            ..Default::default()
        },
        UnitName::DwarvenAxethrower => Unit {
            name: "Dwarven Axethrower".to_string(),
            attack: 2,
            defense: 2,
            movement: 2,
            cost: 5,
            unit_type: hashset! {UnitType::Ranged, UnitType::Infantry},
            support: hashset! {SupportType::Armor{ amount: 1, units: 1}, SupportType::Ranged{ amount: 1, units: 1}},
            terrain_preference: Some(TerrainPreference::Dwarven),
            ..Default::default()
        },
        UnitName::DwarvenHalberdier => Unit {
            name: "Dwarven Halberdier".to_string(),
            attack: 6,
            defense: 4,
            movement: 2,
            cost: 8,
            unit_type: hashset! {UnitType::Melee, UnitType::Infantry},
            support: hashset! {SupportType::Armor{ amount: 3, units: 1}},
            terrain_preference: Some(TerrainPreference::Unyielding),
            ..Default::default()
        },
        UnitName::DwarvenPikeman => Unit {
            name: "Dwarven Pikeman".to_string(),
            attack: 2,
            defense: 6,
            movement: 2,
            cost: 7,
            targeted_attacks: hashset! {TargetedAttack::Formation { amount: 6 }},
            unit_type: hashset! {UnitType::Melee, UnitType::Infantry},
            support: hashset! {SupportType::Armor{ amount: 2, units: 1}},
            terrain_preference: Some(TerrainPreference::Dwarven),
            ..Default::default()
        },
        UnitName::ElvenArcher => Unit {
            name: "Elven Archer".to_string(),
            attack: 2,
            defense: 5,
            movement: 2,
            cost: 6,
            targeted_attacks: hashset! {TargetedAttack::AntiAir { amount: 4}},
            unit_type: hashset! {UnitType::Ranged, UnitType::Infantry, UnitType::Unseen},
            support: hashset! {SupportType::Armor{ amount: 1, units: 1}, SupportType::Ranged{ amount: 2, units: 1}},
            terrain_preference: Some(TerrainPreference::Woodland),
            ..Default::default()
        },
        UnitName::ElvenCavalry => Unit {
            name: "Elven Calvary".to_string(),
            attack: 5,
            defense: 3,
            movement: 4,
            cost: 8,
            targeted_attacks: hashset! {TargetedAttack::Flank { amount: 3}},
            unit_type: hashset! {UnitType::Ranged, UnitType::Cavalry, UnitType::Unseen},
            support: hashset! {SupportType::Ranged{ amount: 2, units: 1}},
            terrain_preference: Some(TerrainPreference::Open),
            ..Default::default()
        },
        UnitName::Fortress => Unit {
            name: "Fortress".to_string(),
            defense: 3,
            defense_rolls: 3,
            max_hp: 2,
            cost: 32,
            unit_type: hashset! {UnitType::Fortification},
            support: hashset! {SupportType::Battlements{ amount: 2, units: 3}},
            terrain_preference: Some(TerrainPreference::Fortification),
            ..Default::default()
        },
        UnitName::NoldorinWarrior => Unit {
            name: "Noldorin Warrior".to_string(),
            attack: 6,
            defense: 4,
            movement: 2,
            cost: 7,
            unit_type: hashset! {UnitType::Melee, UnitType::Infantry, UnitType::Unseen},
            support: hashset! {SupportType::Armor{ amount: 2, units: 1}},
            terrain_preference: Some(TerrainPreference::Woodland),
            ..Default::default()
        },
        UnitName::Raven => Unit {
            name: "Raven".to_string(),
            attack: 4,
            defense: 3,
            movement: 3,
            movement_type: MovementType::Air {
                air_attack: 1,
                air_defense: 1,
            },
            cost: 7,
            unit_type: hashset! {UnitType::Flying, UnitType::Creature},
            terrain_preference: Some(TerrainPreference::Flying),
            ..Default::default()
        },
        UnitName::Raft => Unit {
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
        UnitName::Trebuchet => Unit {
            name: "Trebuchet".to_string(),
            attack: 0,
            defense: 0,
            movement: 2,
            cost: 15,
            targeted_attacks: hashset! {TargetedAttack::Siege {amount: 6, attacks: 3}},
            unit_type: hashset! {UnitType::SiegeEngine},
            support: hashset! {SupportType::Ranged{ amount: 2, units: 2}},
            ..Default::default()
        },
        UnitName::Wall => Unit {
            name: "Wall".to_string(),
            defense: 3,
            max_hp: 2,
            cost: 8,
            unit_type: hashset! {UnitType::Fortification},
            support: hashset! {SupportType::Battlements{ amount: 1, units: 3}},
            terrain_preference: Some(TerrainPreference::Fortification),
            ..Default::default()
        },
    }
}

lazy_static! {
    pub static ref UNITS: HashMap<UnitName, Unit> = UnitName::iter()
        .map(|unit_name| (unit_name.clone(), units(&unit_name)))
        .collect();
}
