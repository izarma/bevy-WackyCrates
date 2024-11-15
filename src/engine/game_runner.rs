use bevy::prelude::*;
use super::player::AddPlayerPlugin;
use super::physics::PhysicsPlugin;

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(AddPlayerPlugin)
        .add_plugins(PhysicsPlugin);
    }
}