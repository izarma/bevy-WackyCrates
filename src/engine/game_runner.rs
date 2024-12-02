use bevy::prelude::*;
use super::player::AddPlayerPlugin;
use super::physics::PhysicsPlugin;
use super::crate_spawner::SpawnCratesPlugin;

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(AddPlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(SpawnCratesPlugin);
    }
}