use bevy::prelude::*;
use crate::GameState;
use super::player::AddPlayerPlugin;

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(AddPlayerPlugin);
    }
}