use bevy::prelude::*;
use bevy::utils::petgraph::data::Element::Node;
use crate::enums::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(start.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(update.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(stop.in_schedule(OnExit(AppState::MainMenu)));
    }
}

fn update(
    input: ResMut<Input<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>
) {
    if input.just_released(KeyCode::Space) {
        next_state.set(AppState::CharacterEdit);
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn start(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // new game
            parent.spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(30.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "New Game",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });

            // load game
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Percent(30.0), Val::Px(65.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Load Game",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });

            // quit game
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Percent(30.0), Val::Px(65.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn stop() {}