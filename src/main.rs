mod enums;
mod main_menu;
mod character_editor;
mod level;

use bevy::prelude::*;
use crate::character_editor::CharacterEditorPlugin;
use crate::enums::AppState;
use crate::main_menu::MainMenuPlugin;

fn main() {
    App::new()
        .add_state::<AppState>() // THIS MUST BE FIRST
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(CharacterEditorPlugin)
        .run();
}
