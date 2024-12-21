use bevy::prelude::*;

use crate::schedule::InGameSet;
use crate::spaceship::{Player, SpaceShip};

fn print_position(query: Query<(Entity, &Transform), (With<SpaceShip>, With<Player>)>) {
    for (entity, transform) in query.iter() {
        info!(
            "Entity {:?} is at position {:?},",
            entity, transform.translation
        );
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position.after(InGameSet::GameLogic));
    }
}
