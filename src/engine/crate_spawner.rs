use crate::engine::physics::*;
use crate::engine::player::*;
use crate::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::time::Duration;

#[derive(Component)]
#[require(Sprite, SpriteSize, Physics)]
pub struct WackyCrate;

#[derive(Resource)]
pub struct SelectionTimer(pub Timer);

pub struct SpawnCratesPlugin;

impl Plugin for SpawnCratesPlugin {
    fn build(&self, app: &mut App) {
        let random_time = Duration::from_millis(rand::random_range(500..5000) as u64);
        app.insert_resource(SelectionTimer(Timer::from_seconds(
            random_time.as_secs_f32(),
            TimerMode::Repeating,
        )))
        .add_systems(Update, spawn_crate.run_if(in_state(GameState::InGame)));
    }
}

fn spawn_crate(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut timer: ResMut<SelectionTimer>,
    time: Res<Time>,
) {
    let texture_handle = asset_server.load("sprites/RTS_Crate.png");
    let window: &Window = window_query.get_single().unwrap();
    let window_width = window.width();
    let random_x = rand::random_range(window_width * -1.0..window_width);
    let frame_size = Vec2::new(51.2, 51.2);
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            WackyCrate,
            Sprite {
                image: texture_handle,
                ..Default::default()
            },
            Transform::from_xyz(random_x, window.height() + 100.0, 0.0)
                .with_scale(Vec3::new(0.1, 0.1, 1.0)),
            Physics::default(),
            SpriteSize::default(),
        ));
    }
}
