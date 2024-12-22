use std::char;

use avian2d::prelude::*;
use bevy::prelude::*;
use bitflags::bitflags;
use rand::Rng;

use crate::schedule::*;
use crate::state::GameState;

#[derive(Component, Debug)]
pub struct Thruster;

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Thrust(f32);

impl Default for Thrust {
    fn default() -> Self {
        Self(0.0)
    }
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Temperature(f32);

impl Default for Temperature {
    fn default() -> Self {
        Self(300.0)
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct ThrusterCharacteristics {
    pub minimum_thrust: f32,
    pub impulse_response: f32,
    pub variance: f32,
    pub max_thrust: f32,
    pub critical_temp: f32, // Temperature at which thruster degrades
    pub faliure_temp: f32,  // Temperature at which thruster breaks
    pub cooling_rate: f32,
    pub heating_rate: f32,
    pub health: f32,
    pub degradation_rate: f32,
}

impl Default for ThrusterCharacteristics {
    fn default() -> Self {
        Self {
            minimum_thrust: 500.0,
            impulse_response: 200.0,
            variance: 100.0,
            max_thrust: 25000.0,
            critical_temp: 500.0,
            faliure_temp: 700.0,
            cooling_rate: 2.0,
            heating_rate: 25.0,
            health: 100.0,
            degradation_rate: 0.01,
        }
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub enum Status {
    #[default]
    Inactive,
    Active,
    Broken,
}

bitflags! {
    #[derive(Component, Debug, Clone, Copy)]
    pub struct ThrusterRoles: u8 {
        const None = 0b00000000;
        const Forward = 0b00000001;
        const Backward = 0b00000010;
        const Left = 0b00000100;
        const Right = 0b00001000;
        const Clockwise = 0b00010000;
        const AntiClockwise = 0b00100000;
        const MainDrive = 0b01000000;
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
    pub characteristics: ThrusterCharacteristics,
    pub roles: ThrusterRoles,
    pub thrust: Thrust,
    pub status: Status,
}

pub fn update_thrusters(
    mut query: Query<(
        &mut Thrust,
        &mut Temperature,
        &mut ThrusterCharacteristics,
        &mut Status,
    )>,
    time: Res<Time>,
) {
    for (mut thrust, mut temperature, mut thruster, mut status) in query.iter_mut() {
        match *status {
            Status::Active => {
                **thrust += thruster.impulse_response * time.delta_secs();
                **thrust += rand::thread_rng().gen_range(-thruster.variance..thruster.variance);
                thrust.0 = thrust.0.clamp(thruster.minimum_thrust, thruster.max_thrust);
                **temperature += thruster.heating_rate * time.delta_secs();
                thruster.health -= thruster.degradation_rate;
                if **temperature > thruster.faliure_temp {
                    thruster.health = 0.0;
                } else if **temperature > thruster.critical_temp {
                    thruster.health -= thruster.degradation_rate * 10.0;
                }
                if thruster.health <= 0.0 {
                    *status = Status::Broken;
                }
            }
            Status::Inactive => {
                **thrust = 0.0;
                let radiative_cooling =
                    thruster.cooling_rate * (temperature.0.powi(4) - 2.7_f32.powi(4));
                **temperature -= radiative_cooling * time.delta_secs();
            }
            Status::Broken => {
                **thrust = 0.0;
                let radiative_cooling =
                    thruster.cooling_rate * (temperature.0.powi(4) - 2.7_f32.powi(4));
                **temperature -= radiative_cooling * time.delta_secs();
            }
        }
    }
}

pub fn apply_force(
    q_thrusters: Query<(&Parent, &Thrust, &Status, &GlobalTransform)>,
    mut q_parent: Query<(&mut ExternalForce, &GlobalTransform, &ComputedCenterOfMass)>,
) {
    for (parent, thrust, status, thruster_global_transform) in q_thrusters.iter() {
        let Ok((mut external_force, &parent_global_transform, &parent_center_of_mass)) =
            q_parent.get_mut(parent.get())
        else {
            return;
        };

        match *status {
            Status::Active => {
                let force = thruster_global_transform
                    .rotation()
                    .mul_vec3(Vec3::Y)
                    .truncate()
                    * **thrust;
                external_force.apply_force_at_point(
                    force,
                    thruster_global_transform.translation().truncate(),
                    parent_global_transform.translation().truncate() + *parent_center_of_mass,
                );
            }
            _ => (),
        };
    }
}

struct ThrusterPlugin;

impl Plugin for ThrusterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, apply_force.in_set(InGameSet::Physics))
            .add_systems(Update, update_thrusters.in_set(InGameSet::GameLogic));
    }
}
