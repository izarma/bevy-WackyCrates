use bevy::prelude::*;
use game_ui::ui_plugin::UiPlugin;

mod animations;
mod consts;
mod engine;
mod game_ui;

#[derive(Debug, Eq, PartialEq, Hash, Resource, States, Default, Clone)]
enum GameState {
    #[default]
    MainMenu,
    Settings,
    InGame,
    _GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(engine::game_runner::GameRunnerPlugin)
        .init_state::<GameState>()
        .add_plugins(UiPlugin)
        .run();
}
