use super::player::AddPlayerPlugin;
use super::player_physics::*;
use crate::animations::asset_loader::*;
use crate::environment::environment_plugin::SpawnEnvironmentsPlugin;
use crate::GameState;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AddPlayerPlugin,
            PlayerPhysicsPlugin,
            SpawnEnvironmentsPlugin,
            PhysicsPlugins::default().with_length_unit(0.00001),
        ))
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .load_collection::<ImageAssets>()
                .continue_to_state(GameState::InGame),
        )
        .insert_resource(Gravity(bevy::prelude::Vec2::new(0.0, -327.0)));
    }
}
