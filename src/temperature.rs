use bevy::prelude::*;

use crate::durability::DurabilityState;
use crate::schedule::InGameSet;

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Temperature(f32);

#[derive(Component, Debug)]
pub struct ThermalCharacteristics {
    critical_temp: f32,
    faliure_temp: f32,
    cooling_rate: f32,
}

impl Default for ThermalCharacteristics {
    fn default() -> Self {
        ThermalCharacteristics {
            critical_temp: 500.0,
            faliure_temp: 700.0,
            cooling_rate: 1.0,
        }
    }
}

impl Default for Temperature {
    fn default() -> Self {
        Temperature(125.0)
    }
}

#[derive(Bundle, Default, Debug)]
pub struct TemperatureBundle {
    pub temperature: Temperature,
    pub characteristics: ThermalCharacteristics,
}

fn update_temperature(
    mut query: Query<(
        &mut Temperature,
        &ThermalCharacteristics,
        &mut DurabilityState,
    )>,
) {
    for (mut temperature, thermal_characteristics, mut system_status) in query.iter_mut() {
        **temperature -= thermal_characteristics.cooling_rate * (**temperature - 2.7_f32.powi(4));
        if **temperature > thermal_characteristics.faliure_temp {
            info!("System broken");
            *system_status = DurabilityState::BROKEN;
        }
        if **temperature > thermal_characteristics.critical_temp {
            *system_status |= DurabilityState::VULNERABLE;
            info!("system vulnerable");
        }
    }
}

pub struct TemperaturePlugin;

impl Plugin for TemperaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_temperature.in_set(InGameSet::Physics));
    }
}
