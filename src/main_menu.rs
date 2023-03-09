use bevy::prelude::*;
use crate::enums::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(main_menu_start.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(main_menu_update.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(main_menu_stop.in_schedule(OnExit(AppState::MainMenu)));
    }
}

fn main_menu_start() {
    println!("Hi");
}

fn main_menu_update(
    input: ResMut<Input<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>
) {
    if input.just_released(KeyCode::Space) {
        next_state.set(AppState::CharacterEdit);
    }
}

fn main_menu_stop() {
    println!("Bye");
}