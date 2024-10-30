use bevy::prelude::*;
use crate::GameState;
use crate::engine::player_animation_state::*;
use crate::engine::sprite_animation::*;
use crate::engine::player_input::*;

pub struct AddPlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    sprite_sheet_bundle: SpriteBundle,
    player: Player,
    state: PlayerState,
    input_state: PlayerInputState,
    anim_state: SpriteAnimState,
}

impl Plugin for AddPlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<PlayerInputs>()
        .add_systems(OnEnter(GameState::InGame),setup_player)
        .add_systems(Update, (keyboard_input,player_movement_state,(animate_sprite,update_player_animation).chain()).run_if(in_state(GameState::InGame)))
        .add_systems(OnExit(GameState::InGame), despawn_player);
    }
}

fn setup_player (mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>) {

    let idle_frames: usize = 6;
    let walk_frames: usize = 10;
    let attack_frames: usize = 4;

    // Define frame sizes
    let frame_size = UVec2::new(128, 128);

    let idle_texture_handle: Handle<Image> = asset_server.load("sprites/City_men_3/Idle.png");
    let walk_texture_handle: Handle<Image> = asset_server.load("sprites/City_men_3/Walk.png");
    let attack_texture_handle: Handle<Image> = asset_server.load("sprites/City_men_3/Attack.png");

    // Create TextureAtlasLayouts
    let idle_layout = TextureAtlasLayout::from_grid(frame_size as UVec2, idle_frames as u32, 1, None, None);
    let idle_layout_handle = texture_atlases.add(idle_layout);

    //let walk_layout = TextureAtlasLayout::from_grid(frame_size, walk_frames as u32, 1, None, None);
    //let walk_layout_handle = texture_atlases.add(walk_layout);

    //let attack_layout = TextureAtlasLayout::from_grid(frame_size, attack_frames as u32, 1, None, None);
    //let attack_layout_handle = texture_atlases.add(attack_layout);

    // Define texture sizes (assuming horizontal sprite sheets)
    let idle_texture_size = Vec2::new(idle_frames as f32 * frame_size.x as f32, frame_size.y as f32);
    let walk_texture_size = Vec2::new(walk_frames as f32 * frame_size.x as f32, frame_size.y as f32);
    let attack_texture_size = Vec2::new(attack_frames as f32 * frame_size.x as f32, frame_size.y as f32);
    
    // Define frame sizes
    let frame_size = UVec2::new(128, 128);

    //Store animations in a resource
    commands.insert_resource(PlayerAnimations {
        idle: Animation {
            frames: idle_frames as usize,
            frame_size,
            texture_size: idle_texture_size,
            texture_handle: idle_texture_handle.clone(),
        },
        walk: Animation {
            frames: walk_frames as usize,
            frame_size,
            texture_size: walk_texture_size,
            texture_handle: walk_texture_handle.clone(),
        },
        attack: Animation {
            frames: attack_frames as usize,
            frame_size,
            texture_size: attack_texture_size,
            texture_handle: attack_texture_handle.clone(),
        },
    });

    commands.spawn(
        (PlayerBundle {
            sprite_sheet_bundle : SpriteBundle {
                texture: idle_texture_handle,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            player: Player,
            state: PlayerState::Idle,
            input_state: PlayerInputState {
                movement_velocity: Vec2::ZERO,
                speed_multiplier: 64.0,
            },
            anim_state: SpriteAnimState {
                start_index: 0,
                end_index: idle_frames - 1,
                frame_size,
                texture_size: idle_texture_size,
                timer: Timer::from_seconds(0.1,TimerMode::Repeating),
            },
        },
        TextureAtlas {
            layout: idle_layout_handle,
            index: 0,
        },
    )
    );

    
}

fn despawn_player(mut commands: Commands, query: Query<Entity, With<SpriteAnimState>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}







