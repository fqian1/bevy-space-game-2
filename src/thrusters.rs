use bevy::prelude::*;

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

#[derive(Bundle)]
pub struct ThrusterBundle {
    pub transform: Transform,
    pub sprite: Sprite,
    pub thrust: Thrust,
    pub fuel_type: FuelType,
    pub fuel_consumption: FuelConsumption,
}

impl Default for ThrusterBundle {
    fn default() -> Self {
        ThrusterBundle {
            transform: Transform::default(),
            sprite: Sprite::default(),
            thrust: Thrust { value: 10.0 },
            fuel_type: FuelType::Hydrazine(1.0),
            fuel_consumption: FuelConsumption { value: 1.0 },
        }
    }
}
