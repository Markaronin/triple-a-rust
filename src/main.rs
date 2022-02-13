use spacegamedata::SpaceGameData;
use spaces::{SpaceName, SPACES};
use std::collections::HashMap;
use strum::IntoEnumIterator;
// use druid::widget::{Button, Flex, Label};
// use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};

mod player;
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
    spaces: HashMap<SpaceName, SpaceGameData>,
}
impl GameState {
    fn new() -> Self {
        GameState {
            phase: TurnPhase::CombatMove,
            spaces: SpaceName::iter()
                .map(|space_name| {
                    (
                        space_name.clone(),
                        SPACES.get(&space_name).unwrap().starting_value.clone(),
                    )
                })
                .into_iter()
                .collect(),
        }
    }
}

fn combat_move(game_state: &mut GameState) {
    println!("Combat move");

    game_state.phase = TurnPhase::BuyUnits;
}
fn buy_units(game_state: &mut GameState) {
    println!("Buy units");
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
