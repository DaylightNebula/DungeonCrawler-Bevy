mod main_menu;
mod enums;

use bevy::prelude::*;
use crate::enums::AppState;
use crate::main_menu::MainMenuPlugin;


fn main() {
    App::new()
        .add_state::<AppState>() // THIS MUST BE FIRST
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        // .add_system_set(SystemSet::on_enter(AppState::Game).with_system(open_game))
        // .add_system_set(SystemSet::on_update(AppState::Game).with_system(update_game))
        // .add_system_set(SystemSet::on_exit(AppState::Game).with_system(exit_game))
        .run();
}

fn open_game() {
    println!("Open game");
}

fn update_game(
    // input: Res<Input<KeyCode>>,
    // mut state: ResMut<State<AppState>>
) {
    // if input.just_released(KeyCode::LControl) {
    //     state.set(AppState::MainMenu).expect("Could not proceed to main menu state");
    // }
}

fn exit_game() {
    println!("Exit game");
}
