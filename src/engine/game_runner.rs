use super::crate_spawner::SpawnCratesPlugin;
use super::physics::PhysicsPlugin;
use super::player::AddPlayerPlugin;
use avian2d::prelude::*;
use bevy::prelude::*;

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AddPlayerPlugin,
            PhysicsPlugin,
            SpawnCratesPlugin,
            PhysicsPlugins::default().with_length_unit(20.0),
        ))
        .insert_resource(Gravity::default());
    }
}
