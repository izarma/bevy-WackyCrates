use crate::animations::sprite_animation::*;
use crate::engine::physics::*;
use crate::engine::player::*;
use crate::engine::player_input::*;
use bevy::prelude::*;

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

    pub fn can_attack(&self) -> bool {
        matches!(
            self.current_state(),
            PlayerStateKind::Idle | PlayerStateKind::Walk(_) | PlayerStateKind::Run(_)
        )
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self(Vec::new())
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerStateKind {
    Idle,
    Walk(Vec2), // Movement vector
    Run(Vec2),  // Movement vector with increased speed
    Jump(Vec2), // Directional jump
    Attack,
    Hurt,
    Dead,
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
                        if !state.0.iter().any(|s| {
                            matches!(s, PlayerStateKind::Walk(_) | PlayerStateKind::Run(_))
                        }) {
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

// Attack combo system
pub fn handle_attack_combo(
    mut query: Query<(&mut PlayerStatus, &PlayerState)>,
    mut input_events: EventReader<PlayerInputs>,
    mut anim_events: EventReader<AnimationEvent>,
) {
    // Handle attack inputs
    for event in input_events.read() {
        if let PlayerInputs::Attack = event {
            for (mut status, state) in &mut query {
                // Only allow attacking if in a valid state
                if state.can_attack() {
                    status.attack_combo = (status.attack_combo + 1) % 3;
                    println!("Attack Combo: {}", status.attack_combo);
                }
            }
        }
    }

    // Handle animation completions
    for event in anim_events.read() {
        if let AnimationEventKind::Finished = event.kind {
            if let Ok((mut status, state)) = query.get_mut(event.entity) {
                // Reset combo when attack animation finishes
                if matches!(state.current_state(), PlayerStateKind::Attack) {
                    status.attack_combo = 0;
                }
            }
        }
    }
}
