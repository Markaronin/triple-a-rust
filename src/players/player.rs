use crate::units::unitname::UnitName;
use druid::Data;
use lazy_static::lazy_static;
use maplit::hashset;
use std::collections::{HashMap, HashSet};
use strum::EnumIter;
use strum::IntoEnumIterator;

#[derive(PartialEq, Eq, Clone, Debug, Hash, EnumIter, Data)]
pub enum PlayerName {
    Saruman,
    Angmar,
    Mordor,
    Arnor,
    Gondor,
    Northmen,
    Lorien,
    Orcs,
    Rhun,
    HighElves,
    WoodlandRealm,
    Harad,
    DolGoldur,
    Freefolk,
    Dwarves,
    Rohan,
    Neutral,
}
impl PlayerName {
    pub fn next_turn(&self) -> PlayerName {
        match self {
            PlayerName::Saruman => PlayerName::Angmar,
            PlayerName::Angmar => PlayerName::Mordor,
            PlayerName::Mordor => PlayerName::Arnor,
            PlayerName::Arnor => PlayerName::Gondor,
            PlayerName::Gondor => PlayerName::Northmen,
            PlayerName::Northmen => PlayerName::Lorien,
            PlayerName::Lorien => PlayerName::Orcs,
            PlayerName::Orcs => PlayerName::Rhun,
            PlayerName::Rhun => PlayerName::HighElves,
            PlayerName::HighElves => PlayerName::WoodlandRealm,
            PlayerName::WoodlandRealm => PlayerName::Harad,
            PlayerName::Harad => PlayerName::DolGoldur,
            PlayerName::DolGoldur => PlayerName::Freefolk,
            PlayerName::Freefolk => PlayerName::Dwarves,
            PlayerName::Dwarves => PlayerName::Rohan,
            PlayerName::Rohan => PlayerName::Saruman,
            PlayerName::Neutral => panic!("It should never be neutral's turn"),
        }
    }
}

pub struct Player {
    pub buyable_units: HashSet<UnitName>,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            buyable_units: HashSet::new(),
        }
    }
}
fn players(player_name: &PlayerName) -> Player {
    match player_name {
        PlayerName::Dwarves => Player {
            buyable_units: hashset! {
                UnitName::DwarvenAxeman,
                UnitName::DwarvenAxethrower,
                UnitName::DwarvenHalberdier,
                UnitName::DwarvenPikeman,
                UnitName::Fortress,
                UnitName::Raft,
                UnitName::Raven,
                UnitName::Trebuchet,
                UnitName::Wall,
            },
        },
        PlayerName::HighElves => Player {
            buyable_units: hashset! {
                UnitName::ElvenArcher,
                UnitName::ElvenCavalry,
                UnitName::NoldorinWarrior,
                UnitName::Wall,
                // TODO - much more than this
            },
        },
        _ => Player {
            ..Default::default()
        },
    }
}
lazy_static! {
    pub static ref PLAYERS: HashMap<PlayerName, Player> = PlayerName::iter()
        .map(|player_name| (player_name.clone(), players(&player_name)))
        .collect();
}
