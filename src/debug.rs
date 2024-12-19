use bevy::prelude::*;

use crate::schedule::InGameSet;
use crate::spaceship::PlayerControlled;
use crate::thrusters::Thrust;

fn print_position(query: Query<(Entity, &Transform), With<Thrust>>) {
    for (entity, transform) in query.iter() {
        info!(
            "Thruster {:?} is at rotation {:?},",
            entity,
            transform.rotation.mul_vec3(Vec3::Y).truncate()
        );
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position.after(InGameSet::GameLogic));
    }
}
