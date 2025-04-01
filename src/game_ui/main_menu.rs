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
    let menu_font = asset_server.load("fonts/Glowdex.ttf");
    let title_img = asset_server.load("WACKY_3.png");
    commands.spawn(Camera2d);
    commands
        .spawn((
            Node {
                align_content: AlignContent::Center,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                position_type: PositionType::Relative,
                flex_wrap: FlexWrap::NoWrap,
                flex_direction: FlexDirection::Column, // Stack items vertically
                justify_content: JustifyContent::FlexStart, // Align from top
                align_items: AlignItems::Center,
                // Add spacing between items
                row_gap: Val::Px(10.0),
                ..Default::default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            // Title Image
            parent.spawn((
                ImageNode {
                    image: title_img,
                    ..default()
                },
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(400.0),
                    margin: UiRect::top(Val::VMin(5.)),
                    ..default()
                },
            ));
            // Start Game Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(consts::BUTTON_BORDER),
                    BorderRadius::MAX,
                    BackgroundColor(consts::NORMAL_BUTTON),
                ))
                .insert(MenuButtons::Play)
                .with_children(|parent: &mut ChildBuilder<'_>| {
                    parent.spawn((
                        Text::from("Play Game"),
                        TextFont {
                            font: menu_font.clone(),
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(consts::TEXT_COLOR),
                    ));
                });
            // Game Settings Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(consts::BUTTON_BORDER),
                    BorderRadius::MAX,
                    BackgroundColor(consts::NORMAL_BUTTON),
                ))
                .insert(MenuButtons::Settings)
                .with_children(|parent| {
                    parent.spawn((
                        Text::from("Settings"),
                        TextFont {
                            font: menu_font,
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(consts::TEXT_COLOR),
                    ));
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
                        game_state.set(GameState::AssetLoading);
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
