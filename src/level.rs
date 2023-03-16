use bevy::prelude::*;
use crate::enums::AppState;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(start.in_schedule(OnEnter(AppState::Level)))
            .add_system(update.in_set(OnUpdate(AppState::Level)))
            .add_system(stop.in_schedule(OnExit(AppState::Level)));
    }
}

fn start(
    mut commands: Commands
) {
}

fn update() {}
fn stop() {}