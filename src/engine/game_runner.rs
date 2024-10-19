use bevy::prelude::*;

struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::InGame),start_game)
        .add_systems(OnExit(GameState::InGame), game_over);
    }
}
fn start_game(mut commands: Commands) {
    commands.spawn().insert(AddPlayerPlugin);
}

fn game_over(mut commands: Commands) {
    commands.remove_resource::<AddPlayerPlugin>();
}