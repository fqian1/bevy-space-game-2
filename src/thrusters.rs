use avian2d::prelude::*;
use bevy::prelude::*;
use bitflags::bitflags;

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Thrust {
    #[deref]
    pub value: f32, // Force applied per second
}

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct FuelConsumption {
    #[deref]
    pub value: f32, // Fuel consumed per second
}

#[derive(Component, Debug)]
pub enum FuelType {
    FusionPellets(f32), // Size of pellet in grams
    Hydrazine(f32),     // Volume in liters
}

#[derive(Component, Debug)]
pub enum Status {
    Active,
    Broken(f32),
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
    }
}

#[derive(Bundle)]
pub struct ThrusterBundle {
    pub transform: Transform,
    pub sprite: Sprite,
    pub thrust: Thrust,
    pub fuel_type: FuelType,
    pub fuel_consumption: FuelConsumption,
    pub thruster_roles: ThrusterRoles,
    pub status: Status,
}

impl Default for ThrusterBundle {
    fn default() -> Self {
        ThrusterBundle {
            transform: Transform::default(),
            sprite: Sprite::default(),
            thrust: Thrust { value: 1000.0 },
            fuel_type: FuelType::Hydrazine(1.0),
            fuel_consumption: FuelConsumption { value: 1.0 },
            thruster_roles: ThrusterRoles::Forward,
            status: Status::Active,
        }
    }
}
