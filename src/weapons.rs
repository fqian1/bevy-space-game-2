use avian2d::prelude::*;
use bevy::prelude::*;

use crate::projectiles::*;
use crate::projectiles::*;
use crate::schedule::InGameSet;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum WeaponType {
    #[default]
    Railgun,
    GatlingGun,
    TorpedoLauncher,
}

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum WeaponState {
    Firing,
    #[default]
    Ready,
    Loading,
}

#[derive(Component, Debug)]
pub struct WeaponCharacteristics {
    rate_of_fire: f32,
    muzzle_location: Transform,
    weapon_power: f32,
    power_consumption: f32,
    spread: f32,
    jam_chance: f32,
    time_since_last_fired: f32,
}

impl Default for WeaponCharacteristics {
    fn default() -> Self {
        Self {
            rate_of_fire: 1.0,
            muzzle_location: Transform {
                translation: Vec3::new(10.0, 10.0, 0.0),
                ..default()
            },
            weapon_power: 100.0,
            power_consumption: 10.0,
            spread: 0.0,
            jam_chance: 0.0,
            time_since_last_fired: 0.0,
        }
    }
}

#[derive(Bundle, Default)]
pub struct WeaponBundle {
    pub weapon_characteristics: WeaponCharacteristics,
    pub weapon_state: WeaponState,
    pub weapon_type: WeaponType,
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

pub fn fire_weapon(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(
        &GlobalTransform,
        &mut WeaponCharacteristics,
        &mut WeaponState,
    )>,
) {
    for (weapon_global_transform, mut weapon_characteristics, mut weapon_status) in query.iter_mut()
    {
        if let WeaponState::Firing = *weapon_status {
            let bullet_bundle = BulletBundle {
                transform: Transform {
                    translation: weapon_global_transform.translation()
                        + weapon_characteristics.muzzle_location.translation,
                    rotation: weapon_global_transform.rotation()
                        + weapon_characteristics.muzzle_location.rotation,
                    ..default()
                },
                ..default()
            };
            commands.spawn((
                bullet_bundle,
                Mesh2d(meshes.add(Rectangle::new(1.0, 2.0))),
                MeshMaterial2d(materials.add(Color::srgb(253.0, 255.0, 23.0))),
            ));
            info!("spawned bullet");
            weapon_characteristics.time_since_last_fired = 0.0;
            *weapon_status = WeaponState::Loading;
        } else if let WeaponState::Loading = *weapon_status {
            if weapon_characteristics.time_since_last_fired >= 1.0 {
                *weapon_status = WeaponState::Ready;
            } else {
                weapon_characteristics.time_since_last_fired += time.delta_secs();
            }
        }
    }
}

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, fire_weapon.in_set(InGameSet::Input));
    }
}
