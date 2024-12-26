use avian2d::prelude::*;
use bevy::prelude::*;

use crate::projectiles::*;
use crate::projectiles::*;
use crate::schedule::InGameSet;
use crate::system_status::SystemStatus;

#[derive(Component, Debug)]
pub struct WeaponCharacteristics {
    rate_of_fire: f32,
    muzzle_location: Vec2,
    weapon_power: f32,
    power_consumption: f32,
    spread: f32,
    jam_chance: f32,
    time_since_last_fired: f32,
}

#[derive(Component, Debug)]
pub struct Railgun;

#[derive(Bundle)]
pub struct RailgunBundle {
    pub marker: Railgun,
    pub transform: Transform,
    pub collider: Collider,
    pub weapon_characteristics: WeaponCharacteristics,
    pub system_status: SystemStatus,
}

impl Default for RailgunBundle {
    fn default() -> Self {
        Self {
            marker: Railgun,
            transform: Transform::default(),
            collider: Collider::rectangle(2.0, 4.0),
            weapon_characteristics: WeaponCharacteristics {
                rate_of_fire: 1.0,
                muzzle_location: Vec2::ZERO,
                weapon_power: 100.0,
                power_consumption: 10.0,
                spread: 0.0,
                jam_chance: 0.0,
                time_since_last_fired: 0.0,
            },
            system_status: SystemStatus::Ready,
        }
    }
}

pub fn fire_railgun(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Transform, &WeaponCharacteristics, &mut SystemStatus)>,
) {
    for (transform, weapon_characteristics, mut system_status) in query.iter_mut() {
        match *system_status {
            SystemStatus::Active => commands.spawn(SlugBundle {
                transform: Transform::from_translation(
                    transform.translation + weapon_characteristics.muzzle_location,
                ),
                external_impulse: External..Default::default(),
            }),
        }
    }
}

#[derive(Component, Debug)]
pub struct GatlingGun;

#[derive(Bundle)]
pub struct GatlingGunBundle {
    pub marker: GatlingGun,
    pub transform: Transform,
    pub collider: Collider,
    pub weapon_characteristics: WeaponCharacteristics,
    pub system_status: SystemStatus,
}

impl Default for GatlingGunBundle {
    fn default() -> Self {
        Self {
            marker: GatlingGun,
            transform: Transform::default(),
            collider: Collider::rectangle(2.0, 4.0),
            weapon_characteristics: WeaponCharacteristics {
                rate_of_fire: 10.0,
                muzzle_location: Vec2::ZERO,
                weapon_power: 10.0,
                power_consumption: 1.0,
                spread: 1.0,
                jam_chance: 0.1,
                time_since_last_fired: 0.0,
            },
            system_status: SystemStatus::Ready,
        }
    }
}

#[derive(Component, Debug)]
pub struct TorpedoLauncher;

#[derive(Bundle)]
pub struct TorpedoLauncherBundle {
    pub marker: TorpedoLauncher,
    pub transform: Transform,
    pub collider: Collider,
    pub weapon_characteristics: WeaponCharacteristics,
    pub system_status: SystemStatus,
}

impl Default for TorpedoLauncherBundle {
    fn default() -> Self {
        Self {
            marker: TorpedoLauncher,
            transform: Transform::default(),
            collider: Collider::rectangle(4.0, 8.0),
            weapon_characteristics: WeaponCharacteristics {
                rate_of_fire: 1.0,
                muzzle_location: Vec2::ZERO,
                weapon_power: 100.0,
                power_consumption: 10.0,
                spread: 0.0,
                jam_chance: 0.0,
                time_since_last_fired: 0.0,
            },
            system_status: SystemStatus::Ready,
        }
    }
}
