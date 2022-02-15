use crate::{players::player::PlayerName, units::unitname::UnitName};
use druid::Data;
use std::collections::HashMap;

#[derive(Clone, Data, PartialEq)]
pub struct PlayerGameData {
    pub money: usize,
    #[data(eq)]
    pub bought_units: HashMap<UnitName, usize>,
}
impl PlayerGameData {
    pub fn starting_value(player_name: &PlayerName) -> Self {
        let money = match player_name {
            PlayerName::Saruman => 38,
            PlayerName::Angmar => 38,
            PlayerName::Mordor => 61,
            PlayerName::Arnor => 31,
            PlayerName::Gondor => 77,
            PlayerName::Northmen => 45,
            PlayerName::Lorien => 26,
            PlayerName::Orcs => 45,
            PlayerName::Rhun => 30,
            PlayerName::HighElves => 39,
            PlayerName::WoodlandRealm => 22,
            PlayerName::Harad => 32,
            PlayerName::DolGoldur => 18,
            PlayerName::Freefolk => 32,
            PlayerName::Dwarves => 37,
            PlayerName::Rohan => 43,
            PlayerName::Neutral => 0,
        };
        Self {
            money,
            bought_units: HashMap::new(),
        }
    }
}
