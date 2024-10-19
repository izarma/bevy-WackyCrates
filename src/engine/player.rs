use bevy::prelude::*;
use crate::GameState;

pub struct AddPlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    sprite_sheet_bundle: SpriteBundle,
    player: Player,
    state: PlayerState,
    input_state: PlayerInputState,
    anim_state: SpriteAnimState,
}

impl Plugin for AddPlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::InGame),setup_player)
        .add_systems(Update, player_input)
        .add_systems(OnExit(GameState::InGame), despawn_player);
    }
}





