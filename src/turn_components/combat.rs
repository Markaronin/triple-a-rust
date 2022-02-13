use crate::{game_state::GameState, turn_components::turn_phase::TurnPhase};

pub fn combat(game_state: &mut GameState) {
    println!("\nCombat");
    // TODO - implement this

    // One round of air combat
    // All targeted attacks
    // Removal of casualties

    // Regular combat
    // Some types of targeted attacks

    // Draw after 10 turns
    game_state.phase = TurnPhase::NonCombatMove;
}
