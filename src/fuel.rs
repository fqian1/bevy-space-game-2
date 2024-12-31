use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct DriveFuelCapacity(pub f32);

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct ThrusterFuelCapacity(pub f32);

#[derive(Bundle)]
pub struct DriveFuelTankBundle {
    drive_fuel_capacity: DriveFuelCapacity,
    thrust_fuel_capacity: ThrusterFuelCapacity,
    collider: Collider,
    mass: Mass,
}
