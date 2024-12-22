use bevy::prelude::*;

use crate::schedule::InGameSet;
use crate::spaceship::{Player, SpaceShip};
use crate::thrusters::{Status, ThrusterRoles};

fn print_position(query: Query<(Entity, &Transform), (With<SpaceShip>, With<Player>)>) {
    for (entity, transform) in query.iter() {
        info!(
            "Entity {:?} is at position {:?},",
            entity, transform.translation
        );
    }
}

fn print_active_thrusters(query: Query<(Entity, &ThrusterRoles, &Status)>) {
    for (entity, roles, status) in query.iter() {
        info!("Thruster: {:?}: {:?}", roles, status);
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, print_position.after(InGameSet::GameLogic));
        // app.add_systems(Update, print_active_thrusters.after(InGameSet::GameLogic));
    }
}
