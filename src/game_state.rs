use crate::{
    players::{player::PlayerName, playergamedata::PlayerGameData},
    spacegamedata::SpaceGameData,
    spaces::SpaceName,
    turn_components::turn_phase::TurnPhase,
};
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub struct GameState {
    pub phase: TurnPhase,
    pub turn: PlayerName,
    pub players: HashMap<PlayerName, PlayerGameData>,
    pub spaces: HashMap<SpaceName, SpaceGameData>,
}
impl GameState {
    pub fn new() -> Self {
        GameState {
            phase: TurnPhase::CombatMove,
            turn: PlayerName::Dwarves,
            players: PlayerName::iter()
                .map(|player_name| {
                    (
                        player_name.clone(),
                        PlayerGameData::starting_value(&player_name),
                    )
                })
                .collect(),
            spaces: SpaceName::iter()
                .map(|space_name| {
                    (
                        space_name.clone(),
                        SpaceGameData::starting_value(&space_name),
                    )
                })
                .into_iter()
                .collect(),
        }
    }
}
