use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FuelType {
    #[default]
    Drive,
    Thrust,
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct FuelTank {
    #[deref]
    current_fuel: f32,
    max_fuel: f32,
}

impl Default for FuelTank {
    fn default() -> Self {
        Self {
            current_fuel: 100.0,
            max_fuel: 100.0,
        }
    }
}

#[derive(Bundle, Default)]
pub struct FuelTankBundle {
    fuel_type: FuelType,
    fuel_tank: FuelTank,
    collider: Collider,
    mass: Mass,
}
