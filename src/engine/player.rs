use crate::engine::physics::*;
use crate::engine::player_animation_state::*;
use crate::engine::player_input::*;
use crate::engine::sprite_animation::*;
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

impl Plugin for AddPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputs>()
            .add_event::<AnimationEvent>()
            .add_systems(OnEnter(GameState::InGame), setup_player)
            .add_systems(
                Update,
                (
                    keyboard_input,
                    player_movement_state,
                    (animate_sprite, update_player_animation).chain(),
                    player_sprite_movement,
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), despawn_player);
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let idle_frames: usize = 13;
    let walk_frames: usize = 10;
    let attack_frames: usize = 6;
    let jump_frames: usize = 10;
    let run_frames: usize = 10;

    // Define frame sizes
    let frame_size = UVec2::new(128, 128);

    let idle_texture_handle: Handle<Image> = asset_server.load("sprites/City_men_2/Idle_2.png");
    let walk_texture_handle: Handle<Image> = asset_server.load("sprites/City_men_2/Walk.png");
    let attack_texture_handle: Handle<Image> = asset_server.load("sprites/City_men_2/Attack_1.png");
    let jump_texture_handle: Handle<Image> = asset_server.load("sprites/City_men_2/Jump.png");
    let run_texture_handle: Handle<Image> = asset_server.load("sprites/City_men_2/Run.png");

    // Create TextureAtlasLayouts
    let idle_layout =
        TextureAtlasLayout::from_grid(frame_size as UVec2, idle_frames as u32, 1, None, None);
    let idle_layout_handle = texture_atlases.add(idle_layout);

    // Define frame sizes
    let frame_size = UVec2::new(128, 128);

    //Store animations in a resource
    commands.insert_resource(PlayerAnimations {
        idle: Animation {
            frames: idle_frames as usize,
            frame_size,
            texture_handle: idle_texture_handle.clone(),
        },
        walk: Animation {
            frames: walk_frames as usize,
            frame_size,
            texture_handle: walk_texture_handle.clone(),
        },
        attack: Animation {
            frames: attack_frames as usize,
            frame_size,
            texture_handle: attack_texture_handle.clone(),
        },
        jump: Animation {
            frames: jump_frames as usize,
            frame_size,
            texture_handle: jump_texture_handle.clone(),
        },
        run: Animation {
            frames: run_frames as usize,
            frame_size,
            texture_handle: run_texture_handle.clone(),
        },
    });

    commands.spawn((
        Player,
        Sprite {
            image: idle_texture_handle,
            texture_atlas: Some(TextureAtlas {
                layout: idle_layout_handle,
                index: 0,
            }),
            ..Default::default()
        },
        PlayerState::default(),
        SpriteAnimState::default(),
        Physics::default(),
        SpriteSize::default(),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn despawn_player(mut commands: Commands, query: Query<Entity, With<SpriteAnimState>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
