use crate::engine::player::*;
use crate::GameState;
use super::env_spawner::*;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Physics {
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub on_ground: bool,
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            on_ground: false,
        }
    }
}

pub struct PlayerPhysicsPlugin;


impl Plugin for PlayerPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (gravity_system, collision_system)
                .chain()
                .run_if(in_state(GameState::InGame)),
        );
    }
}

pub fn gravity_system(mut query: Query<(&mut Physics, &mut Transform)>, time: Res<Time>) {
    for (mut physics, mut transform) in query.iter_mut() {
        if !physics.on_ground {
            physics.acceleration.y = -981.0; // Apply gravity
        } else {
            physics.acceleration.y = 0.0; // No gravity when on the ground
        }
        // Store the computed velocity in a temporary variable to avoid mutable/immutable conflict
        let new_velocity = physics.velocity + physics.acceleration * time.delta_secs();
        physics.velocity = new_velocity;

        // Update the translation with the new velocity
        transform.translation += physics.velocity * time.delta_secs();
        //println!("Physics {:?}", physics);
    }
}

pub fn collision_system(
    mut query: Query<(&mut Physics, &mut Transform, &SpriteSize), Without<Ground>>,
    ground_query: Query<&Ground>,
) {
    let ground = ground_query.single(); // Ensure only one ground

    for (mut physics, mut transform, sprite_size) in query.iter_mut() {
        let ground_level = ground.level + 0.5 * sprite_size.frame_size.y; // Adjust ground level based on player size

        if transform.translation.y <= ground_level && physics.velocity.y <= 0.0 {
            transform.translation.y = ground_level;
            physics.velocity.y = 0.0;
            physics.on_ground = true; // Set to true when on the ground
        } else {
            physics.on_ground = false; // Set to false when in the air
        }
        //println!("Ground {:?}, Physics {:?}, transform {:?}", ground, physics , transform);
    }
}
