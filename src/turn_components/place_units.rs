use crate::{
    spaces::spaces::{SpaceName, SPACES},
    units::unitname::UnitName,
    util::{input, input_int},
    GameState,
};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub fn place_units(game_state: &mut GameState) {
    println!("\nPlace units");

    let current_player = game_state.players.get_mut(&game_state.turn).unwrap();

    let placeable_spaces: HashSet<_> = game_state
        .spaces
        .iter()
        .filter(|(space_name, space_game_data)| {
            let space = SPACES.get(space_name).unwrap();
            space.unit_production > 0 && !space_game_data.conquered_this_turn
        })
        .map(|(space_name, _)| space_name.clone())
        .collect();

    while !current_player.bought_units.is_empty() && placeable_spaces.len() > 0 {
        println!(
            "Placeable units: {placeable_units:?}",
            placeable_units = current_player.bought_units
        );
        println!("You can place in {placeable_spaces:?}");

        let unit_location =
            match SpaceName::from_str(&input("Where would you like to place units?")) {
                Ok(space_name) => {
                    if placeable_spaces.contains(&space_name) {
                        space_name
                    } else {
                        println!("Invalid unit location");
                        continue;
                    }
                }
                Err(_) => {
                    println!("Invalid unit location");
                    continue;
                }
            };
        let unit_type = match UnitName::from_str(&input("Which units would you like to place?")) {
            Ok(unit_name) => {
                // TODO - make sure we can only place units we have bought
                unit_name
            }
            Err(_) => {
                println!("Invalid unit type");
                continue;
            }
        };
        let amount = input_int("How many would you like to place?");

        let current_space = game_state.spaces.get_mut(&unit_location).unwrap();
        // TODO - make sure we can't place more than we have or more than the territory can handle

        *current_player.bought_units.get_mut(&unit_type).unwrap() -= amount;
        if *current_player.bought_units.get_mut(&unit_type).unwrap() == 0 {
            current_player.bought_units.remove(&unit_type);
        }

        current_space.placements_this_turn += amount;

        let unit_game_data = unit_type.default();
        *current_space
            .units
            .entry(unit_type)
            .or_insert(HashMap::new())
            .entry(unit_game_data)
            .or_insert(0) += amount;
    }
}
