use super::crate_spawner::SpawnCratesPlugin;
use super::physics::PhysicsPlugin;
use super::player::AddPlayerPlugin;
use crate::animations::asset_loader::*;
use crate::GameState;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AddPlayerPlugin,
            PhysicsPlugin,
            SpawnCratesPlugin,
            PhysicsPlugins::default().with_length_unit(0.00001),
        ))
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .load_collection::<ImageAssets>()
                .continue_to_state(GameState::InGame),
        )
        .insert_resource(Gravity::default());
    }
}
