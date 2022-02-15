use druid::Data;
use std::collections::HashMap;
use strum::{EnumIter, EnumString};

use super::{unit::UNITS, unitgamedata::UnitGameData};

#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumIter, EnumString, Data)]
pub enum UnitName {
    AncientTower,
    AncientWall,
    DwarvenAxeman,
    DwarvenAxethrower,
    DwarvenHalberdier,
    DwarvenPikeman,
    ElvenArcher,
    ElvenCavalry,
    Fortress,
    NoldorinWarrior,
    Raft,
    Raven,
    Trebuchet,
    Wall,
}
impl UnitName {
    pub fn default(&self) -> UnitGameData {
        let unit = UNITS.get(self).unwrap();
        UnitGameData {
            current_hp: unit.max_hp,
            movement_left: unit.movement,
        }
    }

    pub fn tuple_with_default(self, amount: usize) -> (UnitName, HashMap<UnitGameData, usize>) {
        let default = self.default();
        (self, vec![(default, amount)].into_iter().collect())
    }
}
