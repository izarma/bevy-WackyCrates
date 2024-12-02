use bevy::prelude::*;
use crate::engine::player_input::*;
use crate::engine::player::Player;
use crate::engine::sprite_animation::*;
use crate::engine::physics::*;


#[derive(Component, Debug)]
pub struct PlayerState(pub Vec<PlayerStateKind>);


impl PlayerState {
    pub fn current_state(&self) -> PlayerStateKind {
        *self.0.last().unwrap_or(&PlayerStateKind::Idle)
    }
    pub fn push_state(&mut self, new_state: PlayerStateKind) {
        // Add new state
        self.0.push(new_state);
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerStateKind {
    Idle,
    Walk(Vec2),    // Movement vector
    Run(Vec2),     // Movement vector with increased speed
    Jump(Vec2),    // Directional jump
    Attack,
    _Hurt,
    _Dead,
}

#[derive(Resource)]
pub struct PlayerAnimations {
    pub idle: Animation,
    pub walk: Animation,
    pub attack: Animation,
    pub jump: Animation,
    pub run: Animation,
}

/// Handle player movement state based on user input events and animation finished events.
///
/// For each player input event, this function will add a new state to the player state stack.
/// If the input event is a movement event, it will add a Walk or Run state to the stack.
/// If the input event is a non-movement event (e.g. Attack or Jump), it will add the corresponding state to the stack.
///
/// For each animation finished event, this function will remove the top state from the stack if it is an Attack or Jump state.
/// If the stack is empty after removing the top state, it will add an Idle state to the stack.
pub fn player_movement_state(
    mut player_move_event_reader: EventReader<PlayerInputs>,
    mut player_anim_event_reader: EventReader<AnimationEvent>,
    mut q_player: Query<(&mut PlayerState, &mut Physics), With<Player>>,
) {
    for ev in player_move_event_reader.read() {
        for (mut state, mut physics) in q_player.iter_mut() {
        // Clear idle state before adding a new state
        state.0.retain(|s| !matches!(s, PlayerStateKind::Idle));
        match ev {
            // Player Walk
            PlayerInputs::Walk(direction) => {
                if physics.on_ground {
                let movement = match direction {
                    MoveDirection::Left => Vec2::new(-64.0, 0.0),
                    MoveDirection::Right => Vec2::new(64.0, 0.0),
                };
                state.push_state(PlayerStateKind::Walk(movement));
            }
            }
            // Player Run
            PlayerInputs::Run => {
                if physics.on_ground {
                if let PlayerStateKind::Walk(vel) = state.current_state() {
                    let run_velocity = vel * 2.0; // Increase speed
                    state.push_state(PlayerStateKind::Run(run_velocity));
                }
            }
            }
            // Handle Jump State
            PlayerInputs::Jump => {
                if physics.on_ground {
                let jump_direction = match state.current_state() {
                    PlayerStateKind::Run(vel) | PlayerStateKind::Walk(vel) => vel,
                    _ => Vec2::ZERO,
                };
                let jump_velocity = Vec2::new(jump_direction.x, 64.0); // Upward jump velocity
                state.push_state(PlayerStateKind::Jump(jump_velocity));
            }
            }
            // Handle Attack State
            PlayerInputs::Attack => {
                if physics.on_ground {
                    state.push_state(PlayerStateKind::Attack);
                }
            }
            // Handle WalkEnd and RunEnd if using event-driven approach
            PlayerInputs::WalkEnd(direction) => {
               // Remove the corresponding Walk state from the stack
               state.0.retain(|s| {
                   !(matches!(s, PlayerStateKind::Walk(vel) if (vel.x < 0.0 && *direction == MoveDirection::Left) || (vel.x > 0.0 && *direction == MoveDirection::Right)))
               });
               // Also remove the Run state from the stack
               state.0.retain(|s| !matches!(s, PlayerStateKind::Run(_)));

               if physics.on_ground {
               // Ensure Idle state is on top if no movement states remain
               if !state.0.iter().any(|s| matches!(s, PlayerStateKind::Walk(_) | PlayerStateKind::Run(_))) {
                   state.push_state(PlayerStateKind::Idle);
                   physics.velocity = Vec3::ZERO;
               }
            }
            }
            PlayerInputs::RunEnd => {
                // Modify the top state if it's Run to become Walk
                if physics.on_ground {
                if let Some(top_state) = state.0.last_mut() {
                    if let PlayerStateKind::Run(vel) = *top_state {
                        *top_state = PlayerStateKind::Walk(vel / 2.0);
                    }
                }
                physics.velocity = Vec3::ZERO;
            }
            } 
        }
        println!("Player State: {:#?}", state.0);
        }
    }
    // Clean up states when finished event is triggered
    for event in player_anim_event_reader.read() {
        if let AnimationEventKind::Finished = event.kind {
            for (mut state, mut physics) in q_player.iter_mut() {
                    if let PlayerStateKind::Attack | PlayerStateKind::Jump(_) = state.current_state() {
                        // Pop the Attack state from the stack
                        state.0.pop();
                        if state.0.is_empty() {
                            state.0.push(PlayerStateKind::Idle);
                            physics.velocity.x = 0.0;
                    }
                }
            }
        }
    }
    //println!("Player State: {:#?}", q_player);
}
pub fn update_player_animation(
    player_animations: Res<PlayerAnimations>,
    mut query: Query<(
        &mut Handle<Image>,
        &mut SpriteAnimState,
        &mut Sprite,
        &mut TextureAtlas,
        &PlayerState,
    ), Changed<PlayerState>>,
) {
    for (mut texture_handle, mut anim_state, mut sprite, mut atlas,  state) in query.iter_mut() {
        let animation = match state.current_state() {
            PlayerStateKind::Idle => &player_animations.idle,
            PlayerStateKind::Walk(_) => &player_animations.walk,
            PlayerStateKind::Attack => &player_animations.attack,
            PlayerStateKind::Run(_) => &player_animations.run,
            PlayerStateKind::Jump(_) => &player_animations.jump,
            _ => continue,
        };

        if *texture_handle != animation.texture_handle {
        *texture_handle = animation.texture_handle.clone();
        anim_state.start_index = 0;
        anim_state.frame_size = animation.frame_size;
        anim_state.end_index = animation.frames;
        anim_state.timer = Timer::from_seconds(1.0/12.0, TimerMode::Repeating);
        atlas.index = 0;

        // Reset sprite rect
        sprite.rect = Some(Rect {
            min: Vec2::ZERO,
            max: animation.frame_size.as_vec2(),
        });
    }
    }
}

pub fn player_sprite_movement(

    mut query_player: Query<(&mut Transform, &mut Physics, &PlayerState), With<Player>>,
) {
    for (mut xf, mut physics, state) in query_player.iter_mut() {
 
        match state.current_state() {
            PlayerStateKind::Walk(vel) | PlayerStateKind::Run(vel) => {
                // Update physics velocity
                physics.velocity.x = vel.x;
                // Flip the player's sprite based on the movement direction
                if vel.x < 0.0 {
                    xf.scale.x = -1.0;
                } else {
                    xf.scale.x = 1.0;
                }
            },
            PlayerStateKind::Jump(vel) => {
                physics.velocity.y = vel.y;
            }
            PlayerStateKind::Attack => {
                physics.velocity = Vec3::ZERO;
            },
            _ => {
            },
        }
    }
}

