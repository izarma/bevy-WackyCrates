use avian2d::prelude::*;
use bevy::prelude::*;
use crate::consts;

// Define the collision layers
#[derive(PhysicsLayer, Default, Debug, Clone)]
pub enum PlatformLayer {
    #[default]
    Default,
    Red,
    Blue,
    Green,
}

impl PlatformLayer {
    pub fn get_rgb_color(&self) -> Color {
        match self {
            PlatformLayer::Red => consts::RED,
            PlatformLayer::Blue => consts::BLUE,
            PlatformLayer::Green => consts::GREEN,
            _ => consts::WHITE
        }
    }    
}

#[derive(Component, Debug)]
pub struct Platform {
    pub level: f32, // Represents the Y-level of the ground
    pub layer: PlatformLayer,
}

pub fn spawn_ground(mut commands: Commands) {

    // Red layer
    commands
        .spawn((
            Sprite {
                color: consts::RED,
                custom_size: Some(Vec2::new(4000.0, 20.0)), // Adjust width to fit screen
                ..default()
            },
            Transform::from_xyz(0.0, -150.0, 0.0),
            RigidBody::Static,
            Collider::rectangle(4000.0, 20.0),
            CollisionLayers::new(PlatformLayer::Red, PlatformLayer::Red)
        ))
        .insert(Platform {
            level: -140.0,
            layer: PlatformLayer::Red,
        }); // Set ground level

        // Blue layer
    commands
    .spawn((
        Sprite {
            color: consts::BLUE,
            custom_size: Some(Vec2::new(4000.0, 20.0)), // Adjust width to fit screen
            ..default()
        },
        Transform::from_xyz(0.0, -200.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(4000.0, 20.0),
        CollisionLayers::new(PlatformLayer::Blue, PlatformLayer::Blue)
    ))
    .insert(Platform {
        level: -190.0,
        layer: PlatformLayer::Blue,
    }); // Set ground level

    // Green layer
    commands
        .spawn((
            Sprite {
                color: consts::GREEN,
                custom_size: Some(Vec2::new(4000.0, 20.0)), // Adjust width to fit screen
                ..default()
            },
            Transform::from_xyz(0.0, -250.0, 0.0),
            RigidBody::Static,
            Collider::rectangle(4000.0, 20.0),
            CollisionLayers::new(PlatformLayer::Green, PlatformLayer::Green)
        ))
        .insert(Platform {
            level: -240.0,
            layer: PlatformLayer::Green,
        }); // Set ground level
        
}
