use crate::{player::Player, unitgamedata::UnitGameData, unitname::UnitName};
use std::collections::HashMap;

#[derive(Clone)]
pub struct SpaceGameData {
    pub owner_id: Player,
    pub units: HashMap<UnitName, HashMap<UnitGameData, u64>>,
}
impl Default for SpaceGameData {
    fn default() -> Self {
        Self {
            owner_id: Player::Saruman,
            units: vec![].into_iter().collect(),
        }
    }
}
