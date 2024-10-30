use bevy::prelude::*;
use std::time::Duration;

#[derive(Event, Debug)]
pub enum PlayerInputs {
    Move(Vec2),
    Attack,
}

pub fn keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_move_event: EventWriter<PlayerInputs>,
) {
    let mut movement = Vec2::ZERO;
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        movement.x -= 1.0;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        movement.x += 1.0;
    }
    if movement != Vec2::ZERO {
        player_move_event.send(PlayerInputs::Move(movement));
    } else {
        player_move_event.send(PlayerInputs::Move(Vec2::ZERO));
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        player_move_event.send(PlayerInputs::Attack);
    }
}
