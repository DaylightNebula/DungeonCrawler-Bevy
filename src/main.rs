use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::MainMenu)
        .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(open_main_menu))
        .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(update_main_menu))
        .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(exit_main_menu))
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(open_game))
        .add_system_set(SystemSet::on_update(AppState::Game).with_system(update_game))
        .add_system_set(SystemSet::on_exit(AppState::Game).with_system(exit_game))
        .run();
}

fn open_main_menu() {
    println!("Open main menu");
}

fn update_main_menu(
    input: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>
) {
    if input.just_released(KeyCode::Space) {
        state.set(AppState::Game).expect("Could not proceed to game state");
    }
}

fn exit_main_menu() {
    println!("Exit main menu");
}

fn open_game() {
    println!("Open game");
}

fn update_game(
    input: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>
) {
    if input.just_released(KeyCode::LControl) {
        state.set(AppState::MainMenu).expect("Could not proceed to main menu state");
    }
}

fn exit_game() {
    println!("Exit game");
}
