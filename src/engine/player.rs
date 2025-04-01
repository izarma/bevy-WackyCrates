use crate::animations::asset_loader::ImageAssets;
use crate::animations::player_animation_clips::*;
use crate::animations::player_animation_state::*;
use crate::animations::player_animations::*;
use crate::animations::sprite_animation::*;
use crate::engine::physics::*;
use crate::engine::player_input::*;
use crate::GameState;
use bevy::prelude::*;
pub struct AddPlayerPlugin;

#[derive(Component)]
#[require(Sprite, PlayerState, SpriteAnimState, Physics, SpriteSize)]
pub struct Player;

#[derive(Component)]
pub struct SpriteSize {
    pub frame_size: Vec2,
}

impl Default for SpriteSize {
    fn default() -> Self {
        Self {
            frame_size: Vec2::new(128.0, 128.0),
        }
    }
}

#[derive(Component)]
pub struct PlayerStatus {
    pub hp: u32,
    pub points: u32,
    pub attack_combo: u8,
    pub idle_timer: Timer,
}

impl Default for PlayerStatus {
    fn default() -> Self {
        Self {
            hp: 100,
            points: 0,
            attack_combo: 0,
            idle_timer: Timer::from_seconds(2.0, TimerMode::Once),
        }
    }
}

impl Plugin for AddPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputs>()
            .add_event::<AnimationEvent>()
            .add_systems(
                OnEnter(GameState::InGame),
                (setup_animation_clips, setup_player).chain(),
            )
            .add_systems(
                Update,
                (
                    keyboard_input,
                    player_movement_state,
                    (animate_sprite, update_player_animation, handle_attack_combo).chain(),
                    player_sprite_movement,
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), despawn_player);
    }
}

fn setup_player(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Define frame sizes
    let frame_size = UVec2::new(128, 128);

    // Create TextureAtlasLayouts
    let player_layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
        frame_size as UVec2,
        75 as u32,
        1,
        None,
        None,
    ));

    commands.spawn((
        Player,
        Sprite {
            image: image_assets.player.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: player_layout_handle,
                index: 0,
            }),
            ..Default::default()
        },
        PlayerState::default(),
        SpriteAnimState::default(),
        Physics::default(),
        SpriteSize::default(),
        PlayerStatus::default(),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn despawn_player(mut commands: Commands, query: Query<Entity, With<SpriteAnimState>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
