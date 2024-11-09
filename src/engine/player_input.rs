use bevy::prelude::*;
use std::time::Duration;

#[derive(Event, Debug)]
pub enum PlayerInputs {
    Walk(MoveDirection),
    Run,
    Attack,
    Jump,
    WalkEnd(MoveDirection),
    RunEnd,
}
#[derive(Debug, PartialEq)]
pub enum MoveDirection {
    Left,
    Right,
}

pub fn keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_move_event: EventWriter<PlayerInputs>,
) {

    // handle key presses
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        player_move_event.send(PlayerInputs::Walk(MoveDirection::Left));
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        player_move_event.send(PlayerInputs::Walk(MoveDirection::Right));
    }
    if keyboard_input.just_pressed(KeyCode::ShiftLeft) {
        player_move_event.send(PlayerInputs::Run);
    }
    if keyboard_input.just_pressed(KeyCode::Enter) {
        player_move_event.send(PlayerInputs::Attack);
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
        player_move_event.send(PlayerInputs::Jump);
    }

    // handle key releases

    if keyboard_input.just_released(KeyCode::ArrowLeft) {
        player_move_event.send(PlayerInputs::WalkEnd(MoveDirection::Left));
    }
    if keyboard_input.just_released(KeyCode::ArrowRight) {
        player_move_event.send(PlayerInputs::WalkEnd(MoveDirection::Right));
    }
    if keyboard_input.just_released(KeyCode::ShiftLeft) {
        player_move_event.send(PlayerInputs::RunEnd);
    }
}