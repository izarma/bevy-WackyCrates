use super::environment_plugin::*;
use super::platform_spawner::*;
use crate::animations::asset_loader::ImageAssets;
use crate::engine::player::*;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::sprite::ColorMaterialPlugin;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::ColorImage;

#[derive(Component)]
#[require(Sprite, SpriteSize)]
pub struct WackyCrate;

pub fn spawn_crate(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut timer: ResMut<SelectionTimer>,
    time: Res<Time>,
) {
    let window: &Window = window_query.get_single().unwrap();
    let window_width = window.width();
    let random_x = rand::random_range(-window_width / 2.0..window_width / 2.0);
    let frame_size = Vec2::new(51.2, 51.2);
    if timer.0.tick(time.delta()).just_finished() {
        let random_color = match rand::random_range(0..=2) {
            0 => PlatformLayer::Red,
            1 => PlatformLayer::Blue,
            _ => PlatformLayer::Green,
        };
        commands.spawn((
            WackyCrate,
            Sprite {
                image: image_assets.wacky_crate.clone(),
                color: random_color.get_rgb_color(),
                ..Default::default()
            },
            Transform::from_xyz(random_x, window.height() + 100.0, 0.0)
                .with_scale(Vec3::new(0.1, 0.1, 1.0)),
            SpriteSize { frame_size },
            RigidBody::Dynamic,
            Collider::rectangle(512.0, 512.0),
            ColliderDensity(10.0),
            CollisionLayers::new(random_color.clone(), random_color)
        ));

    }
}
