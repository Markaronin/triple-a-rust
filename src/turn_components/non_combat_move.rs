use crate::{game_state::GameState, turn_components::turn_phase::TurnPhase};

pub fn non_combat_move(game_state: &mut GameState) {
    println!("\nNon combat move");
    // TODO - implement this
    game_state.phase = TurnPhase::PlaceUnits;
}
