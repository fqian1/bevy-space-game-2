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
    muzzle_location: Vec2,
    muzzle_orientation: f32,
    weapon_power: f32,
    power_consumption: f32,
    spread: f32,
    jam_chance: f32,
    time_since_last_fired: f32,
}

impl Default for WeaponCharacteristics {
    fn default() -> Self {
        Self {
            rate_of_fire: 0.01,
            muzzle_location: Vec2::new(0.0, 10.0),
            muzzle_orientation: 0.0,
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
            let bullet_bundle = ProjectileBundle {
                transform: Transform {
                    translation: weapon_global_transform.transform_point(Vec3::new(
                        weapon_characteristics.muzzle_location.x,
                        weapon_characteristics.muzzle_location.y,
                        0.0,
                    )),
                    rotation: weapon_global_transform.rotation()
                        * Quat::from_rotation_z(weapon_characteristics.muzzle_orientation),
                    ..default()
                },
                external_impulse: ExternalImpulse::new(
                    (weapon_global_transform.rotation()
                        * Quat::from_rotation_z(weapon_characteristics.muzzle_orientation))
                    .mul_vec3(Vec3::Y)
                    .truncate()
                        * weapon_characteristics.weapon_power,
                ),
                ..default()
            };
            commands.spawn((
                bullet_bundle,
                Mesh2d(meshes.add(Rectangle::new(1.0, 3.0))),
                MeshMaterial2d(materials.add(Color::srgb(253.0, 255.0, 23.0))),
            ));
            weapon_characteristics.time_since_last_fired = 0.0;
            *weapon_status = WeaponState::Loading;
        } else if let WeaponState::Loading = *weapon_status {
            if weapon_characteristics.time_since_last_fired >= weapon_characteristics.rate_of_fire {
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
