use crate::{game_state::GameState, turn_components::turn_phase::TurnPhase};

pub fn combat_move(game_state: &mut GameState) {
    println!("\nCombat move");

    let movable_spaces = game_state
        .spaces
        .iter()
        .filter(|(_, space_game_data)| space_game_data.owner_id == game_state.turn)
        .filter(|(_, space_game_data)| space_game_data.units.len() > 0)
        .map(|(space_name, _)| space_name)
        .collect::<Vec<_>>();
    println!("You can move from these spaces: {movable_spaces:?}");

    // TODO - implement this

    game_state.phase = TurnPhase::BuyUnits;
}
