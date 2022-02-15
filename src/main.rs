use druid::widget::{Button, Flex, Label, LensWrap};
use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc};
use game_state::GameState;
use players::player::PlayerName;
use spaces_widget::build_spaces_widget;

mod game_state;
mod players;
mod spaces;
mod spaces_widget;
mod terrain;
mod turn_components;
mod units;
mod util;

fn main() -> Result<(), PlatformError> {
    let game_state = GameState::new();
    let main_window = WindowDesc::new(ui_builder()).title("TripleA Rust Clone");
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(game_state)
}

fn ui_builder() -> impl Widget<GameState> {
    let label = Label::new(|data: &PlayerName, _env: &_| format!("Current turn: {:?}", *data))
        .lens(GameState::turn)
        .padding(5.0)
        .center();

    let button = Button::new("Next Turn")
        .on_click(|_ctx, game_state: &mut GameState, _env| {
            game_state.turn = game_state.turn.next_turn()
        })
        .padding(5.0);

    let space = LensWrap::new(build_spaces_widget(), GameState::window_location);

    Flex::column()
        .with_child(label)
        .with_child(button)
        .with_child(space)
}
