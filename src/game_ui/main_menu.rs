use crate::consts;
use crate::GameState;
use bevy::prelude::*;

#[derive(Component)]
pub enum MenuButtons {
    Play,
    Settings,
}

#[derive(Component)]
pub struct OnMainMenuScreen;

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..Default::default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            // Start Game Button
            parent
                .spawn((Button))
                .insert(MenuButtons::Play)
                .with_children(|parent: &mut ChildBuilder<'_>| {
                    parent.spawn(Text::from("Play Game"));
                });
            // Game Settings Button
            parent
                .spawn(Button)
                .insert(MenuButtons::Settings)
                .with_children(|parent| {
                    parent.spawn(Text::from("Settings"));
                });
        });
}

// System to handle button interaction
pub fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButtons),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match button {
                    MenuButtons::Play => {
                        println!("Play Game Button Clicked"); // Switch to Lobby state
                        game_state.set(GameState::InGame);
                    }
                    MenuButtons::Settings => {
                        println!("Settings Button Clicked"); // Switch to Lobby state
                        game_state.set(GameState::Settings);
                    }
                }
            }
            Interaction::Hovered => {
                *color = consts::HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = consts::NORMAL_BUTTON.into();
            }
        }
    }
}

// System to cleanup menu when exiting MainMenu state
pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<OnMainMenuScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
