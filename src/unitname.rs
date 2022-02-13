use crate::{unitgamedata::UnitGameData, unittype::UNIT_TYPES};
use std::collections::HashMap;
use strum::{EnumIter, EnumString};

#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumIter, EnumString)]
pub enum UnitName {
    AncientTower,
    DwarvenPikeman,
    DwarvenAxeman,
    Wall,
}
impl UnitName {
    pub fn default(&self) -> UnitGameData {
        let unit_type_data = UNIT_TYPES.get(self).unwrap();
        UnitGameData {
            current_hp: unit_type_data.max_hp,
            movement_left: unit_type_data.movement,
        }
    }

    pub fn tuple_with_default(self, amount: u64) -> (UnitName, HashMap<UnitGameData, u64>) {
        let default = self.default();
        (self, vec![(default, amount)].into_iter().collect())
    }
}
