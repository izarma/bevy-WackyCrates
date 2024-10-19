use bevy::prelude::*;

struct SpriteAnimState {
    start_index: usize,
    end_index: usize,
    frame_size: UVec2,
    texture_size: Vec2,
    timer: Timer,
}

#[derive(Component, PartialEq, Eq, Debug, Clone, Copy)]
enum PlayerState {
    Idle,
    Walking,
    Running,
    Attacking,
    Hurt,
    Dead,
}

#[derive(Component)]
struct PlayerInputState {
    movement_velocity: Vec2,
    speed_multiplier: f32,
}

fn setup_sprite_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load textures for each animation
    let idle_texture_handle = asset_server.load("sprites/City_men_3/Idle.png");
    let walk_texture_handle = asset_server.load("sprites/City_men_3/Walk.png");
    let attack_texture_handle = asset_server.load("sprites/City_men_3/Attack.png");

    let idle_frames = 6;
    let walk_frames = 10;
    let _run_frames = 10;
    let attack_frames = 4;
    let _hurt_frames = 3;
    let _dead_frames = 5;

     // Define frame sizes
    let frame_size = UVec2::new(128, 128);

    // Create TextureAtlasLayouts
    let idle_layout = TextureAtlasLayout::from_grid(frame_size as UVec2, idle_frames, 1, None, None);
    let idle_layout_handle = texture_atlases.add(idle_layout);

    let walk_layout = TextureAtlasLayout::from_grid(frame_size, walk_frames, 1, None, None);
    let walk_layout_handle = texture_atlases.add(walk_layout);

    let attack_layout = TextureAtlasLayout::from_grid(frame_size, attack_frames, 1, None, None);
    let attack_layout_handle = texture_atlases.add(attack_layout);

     // Define texture sizes (assuming horizontal sprite sheets)
     let idle_texture_size = Vec2::new(idle_frames as f32 * frame_size.x as f32, frame_size.y as f32);
     let walk_texture_size = Vec2::new(walk_frames as f32 * frame_size.x as f32, frame_size.y as f32);
     let attack_texture_size = Vec2::new(attack_frames as f32 * frame_size.x as f32, frame_size.y as f32);

    // Store animations in a resource
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
}

