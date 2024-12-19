use bevy::prelude::*;

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct RateOfFire {
    #[deref]
    pub value: f32, // Time between shots in seconds
}

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct MuzzleVelocity {
    #[deref]
    pub value: f32, // Speed of projectile in m/s
}

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct PowerConsumption {
    #[deref]
    pub value: f32, // Power consumed per shot in watts
}

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Spread {
    #[deref]
    pub value: f32, // Angle of spread in radians
}

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Durability {
    #[deref]
    pub value: u32, // Number of shots before breaking
}

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct HeatCapacity {
    #[deref]
    pub value: f32, // Maximum heat before overheating in joules
}

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct HeatDissipation {
    #[deref]
    pub value: f32, // Heat dissipated per second in joules
}

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct HeatGeneration {
    #[deref]
    pub value: f32, // Heat generated per shot in joules
}

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct JamChance {
    #[deref]
    pub value: f32, // Chance of jamming per shot
}

#[derive(Component)]
pub enum Status {
    Ready,
    Jammed,
    Broken,
}

#[derive(Bundle)]
pub struct GatlingGun {
    pub transform: Transform,
    pub sprite: Sprite,
    pub rate_of_fire: RateOfFire,
    pub muzzle_velocity: MuzzleVelocity,
    pub power_consumption: PowerConsumption,
    pub spread: Spread,
    pub durability: Durability,
    pub heat_capacity: HeatCapacity,
    pub heat_dissipation: HeatDissipation,
    pub heat_generation: HeatGeneration,
    pub jam_chance: JamChance,
    pub status: Status,
}

impl Default for GatlingGun {
    fn default() -> Self {
        GatlingGun {
            transform: Transform::default(),
            sprite: Sprite::default(),
            rate_of_fire: RateOfFire { value: 0.01 },
            muzzle_velocity: MuzzleVelocity { value: 100.0 },
            power_consumption: PowerConsumption { value: 1000.0 },
            spread: Spread { value: 0.1 },
            durability: Durability { value: 1000000 },
            heat_capacity: HeatCapacity { value: 100.0 },
            heat_dissipation: HeatDissipation { value: 0.4 },
            heat_generation: HeatGeneration { value: 0.01 },
            jam_chance: JamChance { value: 0.001 },
            status: Status::Ready,
        }
    }
}

#[derive(Bundle)]
pub struct Railgun {
    pub transform: Transform,
    pub sprite: Sprite,
    pub rate_of_fire: RateOfFire,
    pub muzzle_velocity: MuzzleVelocity,
    pub power_consumption: PowerConsumption,
    pub durability: Durability,
}

#[derive(Bundle)]
pub struct TorpedoTube {
    pub transform: Transform,
    pub sprite: Sprite,
    pub rate_of_fire: RateOfFire,
    pub muzzle_velocity: MuzzleVelocity,
    pub power_consumption: PowerConsumption,
}

#[derive(Bundle)]
pub struct TorpedoEjector {
    pub transform: Transform,
    pub sprite: Sprite,
    pub rate_of_fire: RateOfFire,
    pub muzzle_velocity: MuzzleVelocity,
    pub power_consumption: PowerConsumption,
}
