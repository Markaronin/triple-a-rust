use crate::{
    game_state::GameState,
    players::player::PLAYERS,
    turn_components::turn_phase::TurnPhase,
    units::{unitname::UnitName, unittype::UNIT_TYPES},
    util::{input, input_int},
};
use std::str::FromStr;

pub fn buy_units(game_state: &mut GameState) {
    println!("\nBuy units");

    let current_player = game_state.players.get_mut(&game_state.turn).unwrap();
    let buyable_units = &PLAYERS.get(&game_state.turn).unwrap().buyable_units;

    while current_player.money > 0 {
        println!(
            "Your current money is: {current_money}",
            current_money = current_player.money
        );
        println!("Buyable units: {buyable_units:?}");
        let unit_selection = input("What would you like to buy? (q to stop buying)");
        if unit_selection == "q" {
            break;
        }
        match UnitName::from_str(&unit_selection) {
            Ok(unit_name) => {
                // TODO - make sure we can't buy other faction's units
                let amount = input_int("How many would you like to buy?");
                let price = UNIT_TYPES.get(&unit_name).unwrap().cost * amount;
                if price <= current_player.money {
                    println!("You bought {amount} {unit_name:?}");
                    current_player.money -= UNIT_TYPES.get(&unit_name).unwrap().cost * amount;
                    *current_player.bought_units.entry(unit_name).or_insert(0) += amount;
                } else {
                    println!("You don't have enough money for that")
                }
            }
            Err(_) => println!("Invalid unit"),
        }
    }

    game_state.phase = TurnPhase::Combat;
}
