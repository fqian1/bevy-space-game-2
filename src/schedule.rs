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
        .add_systems(
            Update,
            apply_deferred
                .after(InGameSet::Cleanup)
                .before(InGameSet::Render),
        );
    }
}
