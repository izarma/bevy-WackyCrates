use bevy:: prelude::*;
use crate:: GameState;

#[derive(Component, Debug)]
pub struct Physics {
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

pub struct PhysicsPlugin;

#[derive(Component, Debug)]
pub struct Ground{
    pub level: f32,  // Represents the Y-level of the ground
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::InGame),spawn_ground)
        .add_systems(Update, (gravity_system,collision_system).run_if(in_state(GameState::InGame)));
    }
}



pub fn gravity_system(mut query: Query<(&mut Physics, &mut Transform)>, time: Res<Time>) {
    for (mut physics, mut transform) in query.iter_mut() {
        physics.acceleration.y = -98.1*5.0; //apply gravity
        // Store the computed velocity in a temporary variable to avoid mutable/immutable conflict
        let new_velocity = physics.velocity + physics.acceleration * time.delta_seconds();
        physics.velocity = new_velocity;

        // Update the translation with the new velocity
        transform.translation += physics.velocity * time.delta_seconds();
        //println!("Physics {:?}", physics);
    }
}

pub fn collision_system(
    mut query: Query<(&mut Physics, &mut Transform), Without<Ground>>,
    ground_query: Query<&Ground>,
) {
    let ground = ground_query.single();  // Ensure only one ground
    for (mut physics, mut transform) in query.iter_mut() {
        if transform.translation.y <= ground.level {
            transform.translation.y = ground.level;
            physics.velocity.y = 0.0;
        }
        println!("Ground {:?}, Physics {:?}, transform {:?}", ground, physics , transform);
    }
    
}


pub fn spawn_ground(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(4000.0, 20.0)), // Adjust width to fit screen
                ..default()
            },
            transform: Transform::from_xyz(0.0, -200.0, 0.0),
            ..default()
        })
        .insert(Ground { level: -125.0 });  // Set ground level
}
