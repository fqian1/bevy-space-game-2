use avian2d::prelude::*;
use bevy::prelude::*;
use bitflags::bitflags;
use rand::Rng;

use crate::durability::*;
use crate::schedule::*;
use crate::system_status::SystemStatus;
use crate::temperature::{Temperature, ThermalCharacteristics};

#[derive(Component, Debug)]
pub struct Thruster;

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Thrust(f32);

impl Default for Thrust {
    fn default() -> Self {
        Self(0.0)
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
    pub transform: Transform,
    pub temperature: Temperature,
    pub thermal_characteristics: ThermalCharacteristics,
    pub thruster_characteristics: ThrusterCharacteristics,
    pub roles: ThrusterRoles,
    pub thrust: Thrust,
    pub status: SystemStatus,
    pub durability: Durability,
}

pub fn update_thrusters(
    mut query: Query<(
        &mut Thrust,
        &mut Temperature,
        &mut ThrusterCharacteristics,
        &mut SystemStatus,
    )>,
) {
    for (mut thrust, mut temp, mut thruster, mut status) in query.iter_mut() {
        match *status {
            SystemStatus::Active => {
                **thrust += thruster.impulse_response;
                **thrust = thrust.clamp(thruster.minimum_thrust, thruster.max_thrust);
                **temp += **thrust / 1000.0;
            }
            SystemStatus::Broken => {}
            _ => (),
        }
    }
}

// pub fn update_thrusters(
//     mut query: Query<(
//         &mut Thrust,
//         &mut Temperature,
//         &mut ThrusterCharacteristics,
//         &mut Status,
//     )>,
// ) {
//     for (mut thrust, mut temperature, mut thruster, mut status) in query.iter_mut() {
//         match *status {
//             Status::Active => {
//                 **thrust += thruster.impulse_response;
//                 // **thrust += rand::thread_rng().gen_range(-thruster.variance..thruster.variance);
//                 thrust.0 = thrust.0.clamp(thruster.minimum_thrust, thruster.max_thrust);
//                 **temperature += thruster.heating_rate * **thrust;
//                 thruster.health -= thruster.degradation_rate * **thrust;
//                 if **temperature > thruster.faliure_temp {
//                     thruster.health = 0.0;
//                 } else if **temperature > thruster.critical_temp {
//                     thruster.health -= thruster.degradation_rate * 10.0;
//                 }
//                 if thruster.health <= 0.0 {
//                     *status = Status::Broken;
//                 }
//             }
//             Status::Inactive => {
//                 **thrust = 0.0;
//                 let radiative_cooling =
//                     thruster.cooling_rate * (temperature.0.powi(4) - 2.7_f32.powi(4));
//                 **temperature -= radiative_cooling;
//             }
//             Status::Broken => {
//                 **thrust = 0.0;
//                 let radiative_cooling =
//                     thruster.cooling_rate * (temperature.0.powi(4) - 2.7_f32.powi(4));
//                 **temperature -= radiative_cooling;
//             }
//         }
//     }
// }

// pub fn apply_force(
//     mut q_thrusters: Query<(&Parent, &Thrust, &mut Status, &GlobalTransform)>,
//     mut q_parent: Query<(&mut ExternalForce, &GlobalTransform, &ComputedCenterOfMass)>,
// ) {
//     for (parent, thrust, status, thruster_global_transform) in q_thrusters.iter_mut() {
//         let Ok((mut external_force, parent_global_transform, parent_center_of_mass)) =
//             q_parent.get_mut(parent.get())
//         else {
//             return;
//         };

//         if let SyStatus::Active = *status {
//             let force = thruster_global_transform
//                 .rotation()
//                 .mul_vec3(-Vec3::Y)
//                 .truncate()
//                 * **thrust;
//             external_force.apply_force_at_point(
//                 force,
//                 thruster_global_transform.translation().truncate(),
//                 parent_global_transform.translation().truncate() + **parent_center_of_mass,
//             );
//         }
//     }
// }

pub struct ThrusterPlugin;

impl Plugin for ThrusterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, apply_force.in_set(InGameSet::Physics))
            .add_systems(FixedUpdate, update_thrusters.in_set(InGameSet::GameLogic));
    }
}
