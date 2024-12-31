use avian2d::prelude::*;
use bevy::prelude::*;
use bitflags::bitflags;
use rand::Rng;

use crate::durability::*;
use crate::schedule::*;
use crate::temperature::*;

#[derive(Component, Debug)]
pub struct Thruster;

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Thrust(f32);

impl Default for Thrust {
    fn default() -> Self {
        Self(5000.0)
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct ThrusterCharacteristics {
    pub minimum_thrust: f32, // Minimum thrust that can be produced by the thruster
    pub impulse_response: f32, // Rate of change of thrust per seconds
    pub variance: f32,       // Random noise in thrust
    pub max_thrust: f32,     // Maximum thrust that can be produced by the thruster
}

impl Default for ThrusterCharacteristics {
    fn default() -> Self {
        Self {
            minimum_thrust: 500.0,
            impulse_response: 50000.0,
            variance: 100.0,
            max_thrust: 25000.0,
        }
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub enum ThrusterState {
    Active,
    #[default]
    Ready,
    Inactive,
}

bitflags! {
    #[derive(Component, Debug, Clone, Copy)]
    pub struct ThrusterRoles: u8 {
        const Forward = 0b00000001;
        const Backward = 0b00000010;
        const Left = 0b00000100;
        const Right = 0b00001000;
        const Clockwise = 0b00010000;
        const AntiClockwise = 0b00100000;
        const MainDrive = 0b01000000;
        const None = 0b11111111;
    }
}

impl Default for ThrusterRoles {
    fn default() -> Self {
        Self::Forward
    }
}

#[derive(Bundle, Debug, Default)]
pub struct ThrusterBundle {
    pub thrust: Thrust,
    pub thruster_characteristics: ThrusterCharacteristics,
    pub transform: Transform,
    pub roles: ThrusterRoles,
    pub thruster_status: ThrusterState,
}

pub fn update_thrusters(
    mut query: Query<(
        &mut Thrust,
        &mut Temperature,
        &mut Durability,
        &mut ThrusterCharacteristics,
        &mut ThrusterState,
    )>,
) {
    for (mut thrust, mut temp, mut durability, thruster, status) in query.iter_mut() {
        match *status {
            ThrusterState::Active => {
                **thrust += thruster.impulse_response;
                **thrust = thrust.clamp(thruster.minimum_thrust, thruster.max_thrust);
                // **thrust += rand::thread_rng().gen_range(-thruster.variance..thruster.variance);
                **temp += **thrust / 1000.0;
                **durability -= **thrust / 1000.0;
            }
            _ => {
                **thrust = 0.0;
            }
        }
    }
}

pub fn apply_force(
    mut q_thrusters: Query<(&Parent, &Thrust, &mut ThrusterState, &GlobalTransform)>,
    mut q_parent: Query<(&mut ExternalForce, &GlobalTransform, &ComputedCenterOfMass)>,
) {
    for (parent, thrust, thruster_status, thruster_global_transform) in q_thrusters.iter_mut() {
        let Ok((mut external_force, parent_global_transform, parent_center_of_mass)) =
            q_parent.get_mut(parent.get())
        else {
            return;
        };

        if let ThrusterState::Active = *thruster_status {
            let force = thruster_global_transform
                .rotation()
                .mul_vec3(-Vec3::Y)
                .truncate()
                * **thrust;
            external_force.apply_force_at_point(
                force,
                thruster_global_transform.translation().truncate(),
                parent_global_transform.translation().truncate() + **parent_center_of_mass,
            );
            info!(
                "Applying force {:?} at point {:?}",
                force,
                thruster_global_transform.translation().truncate()
            );
        }
    }
}

pub struct ThrusterPlugin;

impl Plugin for ThrusterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, apply_force.in_set(InGameSet::Physics))
            .add_systems(FixedUpdate, update_thrusters.in_set(InGameSet::GameLogic));
    }
}
