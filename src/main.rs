use game_state::GameState;
use turn_components::{
    buy_units::buy_units, combat::combat, combat_move::combat_move, next_turn::next_turn,
    non_combat_move::non_combat_move, place_units::place_units,
};
// use druid::widget::{Button, Flex, Label};
// use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};

mod game_state;
mod player;
mod playergamedata;
mod spacegamedata;
mod spaces;
mod terrain;
mod turn_components;
mod units;
mod util;

fn main() {
    let mut game_state = GameState::new();
    loop {
        combat_move(&mut game_state);
        buy_units(&mut game_state);
        combat(&mut game_state);
        non_combat_move(&mut game_state);
        place_units(&mut game_state);
        next_turn(&mut game_state);
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
