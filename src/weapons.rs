use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct WeaponSystem {
    rate_of_fire: f32,
    barrel_length: f32,
    power_consumption: f32,
    spread: f32,
    health: f32,
    critical_temp: f32,
    faliure_temp: f32,
    cooling_rate: f32,
    heating_rate: f32,
    degradation_rate: f32,
    jam_chance: f32,
}

impl Default for WeaponSystem {
    fn default() -> Self {
        WeaponSystem {
            rate_of_fire: 0.1,
            barrel_length: 1.0,
            power_consumption: 1000.0,
            spread: 0.1,
            health: 100.0,
            critical_temp: 500.0,
            faliure_temp: 700.0,
            cooling_rate: 2.0,
            heating_rate: 25.0,
            degradation_rate: 0.01,
            jam_chance: 0.001,
        }
    }
}

#[derive(Component)]
pub enum Status {
    Inactive,
    Active,
    Jammed,
    Broken,
}

#[derive(Bundle)]
pub struct GatlingGun {
    pub transform: Transform,
    pub status: Status,
    pub weapon_system: WeaponSystem,
}

impl Default for GatlingGun {
    fn default() -> Self {
        GatlingGun {
            transform: Transform::default(),
            weapon_system: WeaponSystem::default(),
            status: Status::Inactive,
        }
    }
}

#[derive(Bundle)]
pub struct Railgun {
    pub transform: Transform,
    pub weapon_system: WeaponSystem,
}

#[derive(Bundle)]
pub struct TorpedoTube {
    pub transform: Transform,
    pub weapon_system: WeaponSystem,
}

#[derive(Bundle)]
pub struct TorpedoEjector {
    pub transform: Transform,
    pub sprite: Sprite,
}
