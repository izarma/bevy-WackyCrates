use crate::game_ui::main_menu::*;
use crate::game_ui::settings::*;
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                button_interaction_system.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu);
        //.add_systems(OnEnter(GameState::Settings), setup_settings_ui)
        //.add_systems(OnExit(GameState::Settings), cleanup_settings)
        //.init_resource::<SettingsState>()
        //.add_plugins(EguiPlugin);
    }
}
