use crate::{
    game_state::GameState, spaces::spaces::SPACES, turn_components::turn_phase::TurnPhase,
};

pub fn next_turn(game_state: &mut GameState) {
    let current_player = game_state.players.get_mut(&game_state.turn).unwrap();

    game_state
        .spaces
        .iter_mut()
        .filter(|(_, space_game_data)| space_game_data.owner_id == game_state.turn)
        .for_each(|(space_name, space_game_data)| {
            current_player.money += SPACES.get(space_name).unwrap().money_production;
            // Unit operations will only work if we assume units can never be in an area not owned by their owner
            // TODO - set unit movement
            // TODO - set unit HP
            space_game_data.placements_this_turn = 0;
        });

    game_state.turn = game_state.turn.next_turn();

    println!(
        "Start of turn for {current_turn:?}",
        current_turn = game_state.turn
    );

    game_state.phase = TurnPhase::CombatMove;
}
