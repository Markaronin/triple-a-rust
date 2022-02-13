use player::PlayerName;
use playergamedata::PlayerGameData;
use spacegamedata::SpaceGameData;
use spaces::SpaceName;
use std::collections::HashMap;
use strum::IntoEnumIterator;
// use druid::widget::{Button, Flex, Label};
// use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};

mod player;
mod playergamedata;
mod spacegamedata;
mod spaces;
mod terrain;
mod unitgamedata;
mod unitname;
mod unittype;

enum TurnPhase {
    CombatMove,
    BuyUnits,
    Combat,
    NonCombatMove,
    PlaceUnits,
}

struct GameState {
    phase: TurnPhase,
    turn: PlayerName,
    players: HashMap<PlayerName, PlayerGameData>,
    spaces: HashMap<SpaceName, SpaceGameData>,
}
impl GameState {
    fn new() -> Self {
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

fn combat_move(game_state: &mut GameState) {
    println!("Combat move");

    let movable_spaces = game_state
        .spaces
        .iter()
        .filter(|(_, space_game_data)| space_game_data.owner_id == game_state.turn)
        .filter(|(_, space_game_data)| space_game_data.units.len() > 0)
        .map(|(space_name, _)| space_name)
        .collect::<Vec<_>>();
    println!("You can move from these spaces: {movable_spaces:?}");

    game_state.phase = TurnPhase::BuyUnits;
}
fn buy_units(game_state: &mut GameState) {
    println!("Buy units");
    println!(
        "Your current money is: {current_money}",
        current_money = game_state.players.get(&game_state.turn).unwrap().money
    );
    game_state.phase = TurnPhase::Combat;
}
fn combat(game_state: &mut GameState) {
    println!("Combat");
    game_state.phase = TurnPhase::NonCombatMove;
}
fn non_combat_move(game_state: &mut GameState) {
    println!("Non combat move");
    game_state.phase = TurnPhase::PlaceUnits;
}
fn place_units(game_state: &mut GameState) {
    println!("Place units");
    game_state.phase = TurnPhase::CombatMove;
}

fn main() {
    let mut game_state = GameState::new();
    loop {
        combat_move(&mut game_state);
        buy_units(&mut game_state);
        combat(&mut game_state);
        non_combat_move(&mut game_state);
        place_units(&mut game_state);
        break;
    }
    // let main_window = WindowDesc::new(ui_builder);
    // let data = 0_u32;
    // AppLauncher::with_window(main_window)
    //     .use_simple_logger()
    //     .launch(data)
}

// fn ui_builder() -> impl Widget<u32> {
//     // The label text will be computed dynamically based on the current locale and count
//     let text =
//         LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
//     let label = Label::new(text).padding(5.0).center();
//     let button = Button::new("increment")
//         .on_click(|_ctx, data, _env| *data += 1)
//         .padding(5.0);

//     let mut column = Flex::column();
//     column.add_child(label);
//     column.add_child(button);

//     column
// }
