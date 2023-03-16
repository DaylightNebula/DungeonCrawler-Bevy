use bevy::prelude::*;
use crate::enums::AppState;

pub struct CharacterEditorPlugin;

impl Plugin for CharacterEditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(start.in_schedule(OnEnter(AppState::CharacterEdit)))
            .add_system(update.in_set(OnUpdate(AppState::CharacterEdit)))
            .add_system(stop.in_schedule(OnExit(AppState::CharacterEdit)));
    }
}

fn start() {
    println!("Hi");
}

fn update() {

}

fn stop() {
    println!("Bye");
}