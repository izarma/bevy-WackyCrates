use bevy::prelude::*;
use crate::engine::player_input::PlayerInputs;
use crate::engine::player::Player;
use crate::engine::sprite_animation::*;


#[derive(Component, PartialEq, Eq, Debug, Clone, Copy)]
pub enum PlayerState {
    Idle,
    Walking,
    Running,
    Attacking,
    Hurt,
    Dead,
}

#[derive(Component)]
pub struct PlayerInputState {
    pub movement_velocity: Vec2,
    pub speed_multiplier: f32,
}

#[derive(Resource)]
pub struct PlayerAnimations {
    pub idle: Animation,
    pub walk: Animation,
    pub attack: Animation,
}

pub fn player_movement_state(
    mut player_move_event_reader: EventReader<PlayerInputs>,
    mut player_anim_event_reader: EventReader<AnimationEvent>,
    mut q_player: Query<(&mut PlayerState, &mut PlayerInputState), With<Player>>,
) {
    for ev in player_move_event_reader.read() {
        match ev {
            PlayerInputs::Move(vel) => {
                for (mut state, mut input) in q_player.iter_mut() {
                    input.movement_velocity = *vel;
                    if *state != PlayerState::Attacking {
                        if *vel == Vec2::ZERO {
                            *state = PlayerState::Idle;
                        } else {
                            *state = PlayerState::Walking;
                        }
                    }
                }
            }
            PlayerInputs::Attack => {
                for (mut state, _) in q_player.iter_mut() {
                    *state = PlayerState::Attacking;
  
                    }
                }
            }
        // Handle animation events
        for event in player_anim_event_reader.read() {
            if let AnimationEventKind::Finished = event.kind {
                for (mut state, _) in q_player.iter_mut() {
                    if *state == PlayerState::Attacking {
                        *state = PlayerState::Idle;
                    }
                }
            }
        }
    }
}

pub fn update_player_animation(
    player_animations: Res<PlayerAnimations>,
    mut query: Query<(
        &mut Handle<Image>,
        &mut SpriteAnimState,
        &mut Sprite,
        &PlayerState,
    ), With<Player>>,
) {
    for (mut texture_handle, mut anim_state, mut sprite, state) in query.iter_mut() {
        let animation = match *state {
            PlayerState::Idle => &player_animations.idle,
            PlayerState::Walking => &player_animations.walk,
            PlayerState::Attacking => &player_animations.attack,
            _ => continue,
        };

        if *texture_handle != animation.texture_handle {
            *texture_handle = animation.texture_handle.clone();
            anim_state.start_index = 0;
            anim_state.frame_size = animation.frame_size;
            anim_state.texture_size = animation.texture_size;
            anim_state.end_index = animation.frames;
            anim_state.timer = Timer::from_seconds(0.1, TimerMode::Repeating);

            // Reset sprite rect
            sprite.rect = Some(Rect {
                min: Vec2::ZERO,
                max: animation.frame_size.as_vec2(),
            });
        }
    }
}

pub fn player_sprite_movement(
    time: Res<Time>,
    mut query_player: Query<(&mut Transform, &PlayerState, &PlayerInputState), With<Player>>,
) {
    for (mut xf, state, input) in query_player.iter_mut() {
        match state {
            PlayerState::Walking => {
                xf.translation += input.movement_velocity.extend(0.0)
                    * input.speed_multiplier
                    * time.delta_seconds();
            },
            _ => {
            },
        }
    }
}

