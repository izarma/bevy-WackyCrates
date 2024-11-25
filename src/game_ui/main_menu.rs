use bevy::prelude::*;
use crate::consts;
use crate::GameState;

#[derive(Component)]
pub struct PlayGameButton;

#[derive(Component)]
pub struct SettingsButton;

#[derive(Component)]
pub struct OnMainMenuScreen;

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((NodeBundle {
        style: Style{
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..Default::default()
        },
        ..Default::default()
},
OnMainMenuScreen))
.with_children(|parent| {
    // Start Game Button
    parent.spawn(ButtonBundle {
        style: Style {
            margin: UiRect::all(Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Px(200.0),
            height: Val::Px(50.0),
            ..Default::default()
            },
            background_color: consts::NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(PlayGameButton)
        .with_children(|parent: &mut ChildBuilder<'_>| {
            parent.spawn(TextBundle::from_section(
                "Play Game",
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    font: asset_server.load("fonts/Freedom-10eM.ttf"),
                },
            ));
        });
        // Game Settings Button
        parent.spawn(ButtonBundle {
            style: Style {
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                ..Default::default()
                },
                background_color: consts::NORMAL_BUTTON.into(),
                ..Default::default()
            })
            .insert(SettingsButton)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Settings",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        font: asset_server.load("fonts/Freedom-10eM.ttf"),
                    },
                ));
            });
        });
    }


// System to handle button interaction
pub fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&PlayGameButton>, Option<&SettingsButton>),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>
) {
    for (interaction, mut color, host_button, join_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if host_button.is_some() {
                    println!("Play Game Button Clicked");// Switch to Lobby state
                    game_state.set(GameState::InGame);
                } else if join_button.is_some() {
                    println!("Settings Button Clicked");// Switch to Lobby state
                    game_state.set(GameState::Settings);
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