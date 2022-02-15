use crate::{
    players::{player::PlayerName, playergamedata::PlayerGameData},
    spaces::{spacegamedata::SpaceGameData, spaces::SpaceName},
    turn_components::turn_phase::TurnPhase,
    util::{Coord2D, Size2D},
};
use druid::{Data, Lens};
use std::{collections::HashMap, sync::Arc};
use strum::IntoEnumIterator;

#[derive(Clone, Data, Lens)]
pub struct GameState {
    pub window_location: Coord2D,
    pub window_size: Size2D,
    pub phase: TurnPhase,
    pub turn: PlayerName,
    pub players: Arc<HashMap<PlayerName, PlayerGameData>>,
    #[data(eq)]
    pub spaces: Arc<HashMap<SpaceName, SpaceGameData>>,
}
impl GameState {
    pub fn new() -> Self {
        GameState {
            window_location: Coord2D { x: 0, y: 0 },
            window_size: Size2D {
                width: 200,
                height: 200,
            },
            phase: TurnPhase::CombatMove,
            turn: PlayerName::Dwarves,
            players: Arc::new(
                PlayerName::iter()
                    .map(|player_name| {
                        (
                            player_name.clone(),
                            PlayerGameData::starting_value(&player_name),
                        )
                    })
                    .collect(),
            ),
            spaces: Arc::new(
                SpaceName::iter()
                    .map(|space_name| {
                        (
                            space_name.clone(),
                            SpaceGameData::starting_value(&space_name),
                        )
                    })
                    .into_iter()
                    .collect(),
            ),
        }
    }
}
