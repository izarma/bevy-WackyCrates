use bevy:: prelude::*;
use crate::GameState;

#[derive(Component, Debug)]
pub struct Physics {
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

pub struct PhysicsPlugin;

#[derive(Component)]
pub struct Ground;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::InGame),spawn_ground)
        .add_systems(Update, (gravity_system,collision_system).run_if(in_state(GameState::InGame)));
    }
}



pub fn gravity_system(mut query: Query<(&mut Physics, &mut Transform)>, time: Res<Time>) {
    for (mut physics, mut transform) in query.iter_mut() {
        physics.acceleration.y -= 9.81; //apply gravity
         
        if physics.velocity.y == 0.0 {
            physics.acceleration.y = 0.0;
        }
        // Store the computed velocity in a temporary variable to avoid mutable/immutable conflict
        let new_velocity = physics.velocity + physics.acceleration * time.delta_seconds();
        physics.velocity = new_velocity;

        // Update the translation with the new velocity
        transform.translation += physics.velocity * time.delta_seconds();
        println!("Physics {:?}", physics);
    }
}

pub fn collision_system(mut query: Query<(&mut Physics, &mut Transform)>) {
    for (mut physics, mut transform) in query.iter_mut() {
        if transform.translation.y < -10.0 {
            physics.velocity.y = 0.0;
            transform.translation.y = -10.0;
        }
    }
}

pub fn spawn_ground(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -10.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Ground);
}