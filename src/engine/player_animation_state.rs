use bevy::prelude::*;
use crate::engine::player_input::*;
use crate::engine::player::Player;
use crate::engine::sprite_animation::*;


#[derive(Component, Debug)]
pub struct PlayerState(pub Vec<PlayerStateKind>);


impl PlayerState {
    pub fn current_state(&self) -> PlayerStateKind {
        *self.0.last().unwrap_or(&PlayerStateKind::Idle)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerStateKind {
    Idle,
    Walk(Vec2),    // Movement vector
    Run(Vec2),     // Movement vector with increased speed
    Jump(Vec2),    // Directional jump
    Attack,
    Hurt,
    Dead,
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
    mut q_player: Query<(&mut PlayerState), With<Player>>,
) {
    for ev in player_move_event_reader.read() {
        for mut state in q_player.iter_mut() {
        match ev {
            // Player Walk
            PlayerInputs::Walk(direction) => {
                let movement = match direction {
                    MoveDirection::Left => Vec2::new(-64.0, 0.0),
                    MoveDirection::Right => Vec2::new(64.0, 0.0),
                };
                state.0.push(PlayerStateKind::Walk(movement));
            }
            // Player Run
            PlayerInputs::Run => {
                if let PlayerStateKind::Walk(vel) = state.current_state() {
                    let run_velocity = vel * 2.0; // Increase speed
                    state.0.push(PlayerStateKind::Run(run_velocity));
                }
            }
            // Handle Jump State
            PlayerInputs::Jump => {
                let jump_direction = match state.current_state() {
                    PlayerStateKind::Run(vel) | PlayerStateKind::Walk(vel) => vel,
                    _ => Vec2::ZERO,
                };
                state.0.push(PlayerStateKind::Jump(jump_direction));
            }
            // Handle Attack State
            PlayerInputs::Attack => {
                state.0.push(PlayerStateKind::Attack);
            }
            // Handle WalkEnd and RunEnd if using event-driven approach
            PlayerInputs::WalkEnd(direction) => {
               // Remove the corresponding Walk state from the stack
               state.0.retain(|s| {
                   !(matches!(s, PlayerStateKind::Walk(vel) if (vel.x < 0.0 && *direction == MoveDirection::Left) || (vel.x > 0.0 && *direction == MoveDirection::Right)))
               });
               // Ensure Idle state is on top if no movement states remain
               if !state.0.iter().any(|s| matches!(s, PlayerStateKind::Walk(_) | PlayerStateKind::Run(_))) {
                   state.0.push(PlayerStateKind::Idle);
               }
            }
            PlayerInputs::RunEnd => {
                // Modify the top state if it's Run to become Walk
                if let Some(top_state) = state.0.last_mut() {
                    if let PlayerStateKind::Run(vel) = *top_state {
                        *top_state = PlayerStateKind::Walk(vel / 2.0);
                    }
                }
            }
        }
        println!("Player State: {:#?}", state.0);
        }
    }
    // Clean up states when finished event is triggered
    for event in player_anim_event_reader.read() {
        if let AnimationEventKind::Finished = event.kind {
            for mut state in q_player.iter_mut() {
                    if let PlayerStateKind::Attack = state.current_state() {
                        // Pop the Attack state from the stack
                        state.0.pop();
                        if state.0.is_empty() {
                            state.0.push(PlayerStateKind::Idle);
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
        let animation = match state.current_state() {
            PlayerStateKind::Idle => &player_animations.idle,
            PlayerStateKind::Walk(_) => &player_animations.walk,
            PlayerStateKind::Attack => &player_animations.attack,
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
    mut query_player: Query<(&mut Transform, &PlayerState), With<Player>>,
) {
    for (mut xf, state) in query_player.iter_mut() {
        match state.current_state() {
            PlayerStateKind::Walk(vel) | PlayerStateKind::Run(vel) => {
                xf.translation += vel.extend(0.0)
                    * time.delta_seconds();
            },
            _ => {
            },
        }
    }
}

