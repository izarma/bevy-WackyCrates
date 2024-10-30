use bevy::prelude::*;
use game_ui::ui_plugin::UiPlugin;

mod game_ui;
mod consts;
mod engine;

#[derive(Debug, Eq, PartialEq, Hash, Resource, States, Default, Clone)]
enum GameState {
    #[default]
    MainMenu,
    Settings,
    InGame,
    GameOver,
}
 
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(engine::game_runner::GameRunnerPlugin)
        .init_state::<GameState>()
        .add_plugins(UiPlugin)
        .run();
}