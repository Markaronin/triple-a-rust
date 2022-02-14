use crate::{game_state::GameState, turn_components::turn_phase::TurnPhase, util::input};
use std::collections::HashSet;

pub fn combat_move(game_state: &mut GameState) {
    println!("\nCombat move");

    let movable_spaces = game_state
        .spaces
        .iter()
        .filter(|(_, space_game_data)| space_game_data.owner_id == game_state.turn)
        .filter(|(_, space_game_data)| space_game_data.units.len() > 0)
        .map(|(space_name, _)| space_name)
        .collect::<HashSet<_>>();
    println!("You can move from these spaces: {movable_spaces:?}");

    let space_to_move = input("Where would you like to move? (q to stop moving)");
    if space_to_move != "q" {}

    // TODO - implement this

    game_state.phase = TurnPhase::BuyUnits;
}
