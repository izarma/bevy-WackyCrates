use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Debug)]
struct PlayerInputState {
    movement_velocity: Vec2,
    speed_multiplier: f32,
}

#[derive(Event, Debug)]
enum PlayerInputs {
    Move(Vec2),
    Attack,
}
