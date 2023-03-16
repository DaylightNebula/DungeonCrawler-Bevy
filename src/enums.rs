use bevy::prelude::*;

#[derive(Default, States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    MainMenu,
    CharacterEdit,
    Level,
}