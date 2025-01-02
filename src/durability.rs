use bevy::prelude::*;
use bitflags::bitflags;

use crate::schedule::InGameSet;

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Durability(f32);

impl Default for Durability {
    fn default() -> Self {
        Durability(100.0)
    }
}

bitflags! {
    #[derive(Component, Debug, Default, Clone, Copy)]
    pub struct DurabilityState: u8 {
        const HEALTHY = 0b0001;
        const BROKEN = 0b0010;
        const VULNERABLE = 0b0100;
        const REPAIRING = 0b1000;
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

#[derive(Bundle, Default, Debug)]
pub struct DurabilityBundle {
    pub durability: Durability,
    pub characteristics: DurabilityCharacteristics,
}

fn update_durability(
    mut query: Query<(
        &mut Durability,
        &DurabilityCharacteristics,
        &mut DurabilityState,
    )>,
) {
    for (mut durability, durability_characteristics, mut durability_state) in query.iter_mut() {
        if durability_state.contains(DurabilityState::REPAIRING) {
            **durability += durability_characteristics.repair_rate;
            if **durability >= durability_characteristics.max_durability {
                **durability = durability_characteristics.max_durability;
                *durability_state &= !DurabilityState::REPAIRING;
            }
        } else if **durability <= 0.0 {
            *durability_state = DurabilityState::BROKEN;
        }
    }
}

pub struct DurabilityPlugin;

impl Plugin for DurabilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_durability.in_set(InGameSet::Physics));
    }
}
