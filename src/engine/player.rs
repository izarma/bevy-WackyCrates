use bevy::prelude::*;
use crate::GameState;
use crate::engine::player_animation_state::*;
use crate::engine::sprite_animation::*;
use crate::engine::player_input::*;
use crate::engine::physics::*;
pub struct AddPlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct SpriteSize{
    pub frame_size: Vec2,
}

#[derive(Bundle)]
struct PlayerBundle {
    sprite_sheet_bundle: SpriteBundle,
    player: Player,
    state: PlayerState,
    anim_state: SpriteAnimState,
    texture: TextureAtlas,
    physics: Physics,
    frame_size: SpriteSize,
}

impl Plugin for AddPlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<PlayerInputs>()
        .add_event::<AnimationEvent>()
        .add_systems(OnEnter(GameState::InGame),setup_player)
        .add_systems(Update, (keyboard_input,player_movement_state,(animate_sprite,update_player_animation,).chain(),player_sprite_movement).run_if(in_state(GameState::InGame)))
        .add_systems(OnExit(GameState::InGame), despawn_player);
    }
}

fn setup_player (mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>) {

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
    let idle_layout = TextureAtlasLayout::from_grid(frame_size as UVec2, idle_frames as u32, 1, None, None);
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

    commands.spawn(
        PlayerBundle {
            sprite_sheet_bundle : SpriteBundle {
                texture: idle_texture_handle,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            player: Player,
            state: PlayerState(vec![PlayerStateKind::Idle]),
            anim_state: SpriteAnimState {
                start_index: 0,
                end_index: idle_frames - 1,
                frame_size,
                timer: Timer::from_seconds(1.0/12.0,TimerMode::Repeating),
            },
            texture: TextureAtlas {
                layout: idle_layout_handle,
                index: 0,
            },
            physics: Physics {
                velocity: Vec3::ZERO,
                acceleration: Vec3::ZERO,
                on_ground: false
            },
            frame_size: SpriteSize{
                frame_size:frame_size.as_vec2()},
        },
    );

    
}

fn despawn_player(mut commands: Commands, query: Query<Entity, With<SpriteAnimState>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}







