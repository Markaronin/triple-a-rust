use crate::{players::player::PlayerName, units::unitname::UnitName};
use std::collections::HashMap;

pub struct PlayerGameData {
    pub money: u64,
    pub bought_units: HashMap<UnitName, u64>,
}
impl PlayerGameData {
    pub fn starting_value(player_name: &PlayerName) -> Self {
        let money = match player_name {
            PlayerName::Dwarves => 38,
            _ => 0,
        };
        Self {
            money,
            bought_units: HashMap::new(),
        }
    }
}