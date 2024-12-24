use bevy::prelude::*;

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct DriveFuelCapacity(pub f32);

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct ThrusterFuelCapacity(pub f32);
