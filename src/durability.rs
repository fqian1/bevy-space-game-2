use bevy::prelude::*;

use crate::schedule::InGameSet;
use crate::system_status::SystemStatus;

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Durability(pub f32);

impl Default for Durability {
    fn default() -> Self {
        Durability(100.0)
    }
}

#[derive(Component, Debug)]
pub struct DurabilityCharacteristics {
    pub max_durability: f32,
    pub repair_rate: f32,
}

impl Default for DurabilityCharacteristics {
    fn default() -> Self {
        DurabilityCharacteristics {
            max_durability: 100.0,
            repair_rate: 0.1,
        }
    }
}

#[derive(Bundle, Default)]
pub struct DurabilityBundle {
    pub durability: Durability,
    pub characteristics: DurabilityCharacteristics,
}

fn update_durability(
    mut query: Query<(
        &mut Durability,
        &DurabilityCharacteristics,
        &mut SystemStatus,
    )>,
) {
    for (mut durability, durability_characteristics, mut system_status) in query.iter_mut() {
        match *system_status {
            SystemStatus::Repairing => {
                **durability += durability_characteristics.repair_rate;
                if **durability >= durability_characteristics.max_durability {
                    **durability = durability_characteristics.max_durability;
                    *system_status = SystemStatus::Ready;
                }
            }
            _ => {
                if **durability <= 0.0 {
                    *system_status = SystemStatus::Broken;
                    info!("system broken durability");
                }
            }
        }
    }
}

pub struct DurabilityPlugin;

impl Plugin for DurabilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_durability.in_set(InGameSet::Physics));
    }
}
