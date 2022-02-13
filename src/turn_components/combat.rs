use crate::{game_state::GameState, turn_components::turn_phase::TurnPhase};

pub fn combat(game_state: &mut GameState) {
    println!("\nCombat");
    // TODO - implement this
    game_state.phase = TurnPhase::NonCombatMove;
}
