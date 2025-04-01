use crate::animations::player_animation_state::*;
use crate::animations::sprite_animation::*;
use crate::engine::physics::*;
use crate::engine::player::Player;
use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerAnimations {
    pub idle: Animation,
    pub walk: Animation,
    pub attack: Animation,
    pub jump: Animation,
    pub run: Animation,
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
            }
            PlayerStateKind::Jump(vel) => {
                physics.velocity.y = vel.y;
            }
            PlayerStateKind::Attack => {
                physics.velocity = Vec3::ZERO;
            }
            _ => {}
        }
    }
}
