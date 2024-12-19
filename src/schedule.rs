use bevy::prelude::*;

use crate::state::GameState;

#[derive(Debug, Clone, Hash, PartialEq, Eq, SystemSet)]
pub enum InGameSet {
    Input,
    Physics,
    Collisions,
    GameLogic,
    Cleanup,
    Render,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, SystemSet)]
pub enum MainMenuSet {
    Input,
    Render,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, SystemSet)]
pub enum GameOverSet {
    Input,
    Render,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, SystemSet)]
pub enum LoadingSet {
    Render,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::Input,
                InGameSet::Physics,
                InGameSet::Collisions,
                InGameSet::GameLogic,
                InGameSet::Cleanup,
                InGameSet::Render,
            )
                .chain()
                .run_if(in_state(GameState::InGame)),
        )
        .configure_sets(
            Update,
            (MainMenuSet::Input, MainMenuSet::Render)
                .chain()
                .run_if(in_state(GameState::MainMenu)),
        )
        .configure_sets(
            Update,
            (GameOverSet::Input, GameOverSet::Render)
                .chain()
                .run_if(in_state(GameState::GameOver)),
        )
        .configure_sets(
            Update,
            LoadingSet::Render.run_if(in_state(GameState::Loading)),
        );
    }
}
