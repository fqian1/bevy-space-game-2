use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use serde::{Deserialize, Serialize};

use crate::asset_loader::ImageAssets;
use crate::fuel::*;
use crate::particle_effects::setup;
use crate::particle_effects::*;
use crate::schedule::InGameSet;
use crate::thrusters::ThrusterState;
use crate::thrusters::*;
use crate::weapons::*;

#[derive(Component, Debug)]
pub struct SpaceShip;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Ai;

#[derive(Bundle)]
pub struct SpaceshipBundle {
    // pub thruster_fuel: FuelTankBundle,
    // pub drive_fuel: FuelTankBundle,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub transform: Transform,
    pub external_force: ExternalForce,
    pub sleeping_disabled: SleepingDisabled,
}

impl Default for SpaceshipBundle {
    fn default() -> Self {
        Self {
            // thruster_fuel: FuelTankBundle::default(),
            // drive_fuel: FuelTankBundle::default(),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::rectangle(15.0, 25.0),
            transform: Transform::from_translation(Vec3::ZERO),
            external_force: ExternalForce::new(Vec2::ZERO).with_persistence(false),
            sleeping_disabled: SleepingDisabled,
        }
    }
}

fn spawn_player_spaceship(
    mut commands: Commands,
    assets: Res<ImageAssets>,
    effect_handle: Res<EffectAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let rectangle = Rectangle::new(15.0, 25.0);

    let spaceship = commands
        .spawn((
            Player,
            SpaceShip,
            SpaceshipBundle {
                collider: rectangle.collider(),
                ..default()
            },
            Mesh2d(meshes.add(rectangle)),
            MeshMaterial2d(materials.add(Color::srgb(0.8, 0.8, 0.8))),
        ))
        .id();

    let thruster_locations = [
        Vec2::new(0.0, -12.5),  // Main Drive
        Vec2::new(-7.5, -12.5), // Bottom left
        Vec2::new(7.5, -12.5),  // Bottom right
        Vec2::new(-7.5, 12.5),  // Top left
        Vec2::new(7.5, 12.5),   // Top right
    ];

    let thruster_rotations = [
        std::f32::consts::PI,              // Bottom
        std::f32::consts::FRAC_PI_4 * 3.0, // Bottom left
        std::f32::consts::FRAC_PI_4 * 5.0, // Bottom right
        std::f32::consts::FRAC_PI_4,       // Top Left
        std::f32::consts::FRAC_PI_4 * 7.0, // Top Right
    ];

    let thruster_roles = [
        ThrusterRoles::MainDrive,
        ThrusterRoles::Forward | ThrusterRoles::Right | ThrusterRoles::AntiClockwise,
        ThrusterRoles::Forward | ThrusterRoles::Left | ThrusterRoles::Clockwise,
        ThrusterRoles::Backward | ThrusterRoles::Right | ThrusterRoles::Clockwise,
        ThrusterRoles::Backward | ThrusterRoles::Left | ThrusterRoles::AntiClockwise,
    ];

    let thruster_characteristics = [
        ThrusterCharacteristics {
            minimum_thrust: 2500.0,
            impulse_response: 500000.0,
            variance: 100.0,
            max_thrust: 250000.0,
        },
        ThrusterCharacteristics {
            minimum_thrust: 500.0,
            impulse_response: 50000.0,
            variance: 100.0,
            max_thrust: 25000.0,
        },
        ThrusterCharacteristics {
            minimum_thrust: 500.0,
            impulse_response: 50000.0,
            variance: 100.0,
            max_thrust: 25000.0,
        },
        ThrusterCharacteristics {
            minimum_thrust: 500.0,
            impulse_response: 50000.0,
            variance: 100.0,
            max_thrust: 25000.0,
        },
        ThrusterCharacteristics {
            minimum_thrust: 500.0,
            impulse_response: 50000.0,
            variance: 100.0,
            max_thrust: 25000.0,
        },
    ];

    let thrusters: Vec<Entity> = (0..5)
        .map(|i| {
            commands
                .spawn((
                    ThrusterBundle {
                        transform: Transform::from_translation(thruster_locations[i].extend(0.0))
                            .with_rotation(Quat::from_rotation_z(thruster_rotations[i])),
                        roles: thruster_roles[i],
                        thruster_characteristics: thruster_characteristics[i],
                        ..default()
                    },
                    Mesh2d(meshes.add(Rectangle::new(2.0, 2.0))),
                    MeshMaterial2d(materials.add(Color::srgb(0.8, 0.8, 0.8))),
                ))
                .with_children(|parent| {
                    parent.spawn(ParticleEffectBundle {
                        effect: ParticleEffect::new(effect_handle.thruster_gas.clone()),
                        ..default()
                    });
                })
                .id()
        })
        .collect();

    let gatling_gun = commands
        .spawn((
            WeaponBundle {
                weapon_type: WeaponType::GatlingGun,
                transform: Transform::from_translation(Vec3::new(10.0, 0.0, 0.0))
                    .with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2)),
                weapon_state: WeaponState::Ready,
                ..default()
            },
            Mesh2d(meshes.add(Rectangle::new(3.0, 6.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.2, 0.3, 0.4))),
        ))
        .id();

    commands
        .entity(spaceship)
        .add_children(&thrusters)
        .add_child(gatling_gun);
}

fn spaceship_control(
    q_spaceship: Query<&Children, (With<Player>, With<SpaceShip>)>,
    mut q_thrusters: Query<(&mut ThrusterState, &ThrusterRoles)>,
    mut q_weapons: Query<(&mut WeaponState, &WeaponType)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(children) = q_spaceship.get_single() else {
        return;
    };

    let mut set_weapon_state = |weapon_type: &WeaponType| {
        for &child in children.iter() {
            let Ok((mut weapon_state, q_weapon_type)) = q_weapons.get_mut(child) else {
                continue;
            };
            if weapon_type == q_weapon_type && *weapon_state == WeaponState::Ready {
                *weapon_state = WeaponState::Firing;
            }
        }
    };

    let mut set_thruster_status = |role: &ThrusterRoles| {
        for &thruster in children.iter() {
            let Ok((mut status, thruster_role)) = q_thrusters.get_mut(thruster) else {
                return;
            };
            if role.contains(ThrusterRoles::None) {
                *status = ThrusterState::Inactive;
                continue;
            }
            if thruster_role.contains(*role) {
                *status = ThrusterState::Active;
            }
        }
    };

    if keyboard_input.pressed(KeyCode::KeyW) {
        set_thruster_status(&ThrusterRoles::Forward);
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        set_thruster_status(&ThrusterRoles::Left);
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        set_thruster_status(&ThrusterRoles::Backward);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        set_thruster_status(&ThrusterRoles::Right);
    }
    if keyboard_input.pressed(KeyCode::Space) {
        set_thruster_status(&ThrusterRoles::MainDrive);
    }
    if keyboard_input.pressed(KeyCode::KeyE) {
        set_thruster_status(&ThrusterRoles::Clockwise);
    }
    if keyboard_input.pressed(KeyCode::KeyQ) {
        set_thruster_status(&ThrusterRoles::AntiClockwise);
    }
    if keyboard_input.pressed(KeyCode::KeyF) {
        set_weapon_state(&WeaponType::GatlingGun);
    }
    // if keyboard_input.pressed(KeyCode::KeyR) {
    //     set_status(&ThrusterRoles::Forward);
    // }
    // if keyboard_input.pressed(KeyCode::KeyT) {
    //     set_status(&ThrusterRoles::Forward);
    // }
    // if keyboard_input.pressed(KeyCode::KeyG) {
    //     set_status(&ThrusterRoles::Forward);
    // }
    if keyboard_input.any_just_released([
        KeyCode::KeyW,
        KeyCode::KeyA,
        KeyCode::KeyS,
        KeyCode::KeyD,
        KeyCode::Space,
        KeyCode::KeyE,
        KeyCode::KeyQ,
    ]) {
        set_thruster_status(&ThrusterRoles::None);
    }
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player_spaceship.after(setup))
            .add_systems(FixedUpdate, spaceship_control.in_set(InGameSet::Physics));
    }
}
