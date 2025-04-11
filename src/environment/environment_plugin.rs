use super::crate_spawner::*;
use super::platform_spawner::*;
use crate::GameState;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource)]
pub struct SelectionTimer(pub Timer);

pub struct SpawnEnvironmentsPlugin;

impl Plugin for SpawnEnvironmentsPlugin {
    fn build(&self, app: &mut App) {
        let random_time = Duration::from_millis(rand::random_range(500..3000) as u64);
        app.insert_resource(SelectionTimer(Timer::from_seconds(
            random_time.as_secs_f32(),
            TimerMode::Repeating,
        )))
        .add_systems(OnEnter(GameState::InGame), spawn_ground)
        .add_systems(Update, spawn_crate.run_if(in_state(GameState::InGame)));
    }
}
