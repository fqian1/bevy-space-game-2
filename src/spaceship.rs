use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::asset_loader::ImageAssets;
use crate::controller::PlayerInputEvents;
use crate::schedule::InGameSet;
use crate::state::GameState;
use crate::thrusters::Status;
use crate::thrusters::*;
use crate::weapons::*;

#[derive(Component, Debug)]
pub struct SpaceShip;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Ai;

#[derive(Component, Debug)]
pub struct Inactive;

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Health {
    #[deref]
    pub value: f32,
}

#[derive(Bundle)]
pub struct SpaceshipBundle {
    pub spaceship: SpaceShip,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub transform: Transform,
    pub external_force: ExternalForce,
    pub sleeping_disabled: SleepingDisabled,
}

impl Default for SpaceshipBundle {
    fn default() -> Self {
        let shape = Rectangle::new(15.0, 25.0);
        Self {
            spaceship: SpaceShip,
            rigid_body: RigidBody::Dynamic,
            collider: Collider::default(),
            transform: Transform::from_translation(Vec3::ZERO),
            external_force: ExternalForce::new(Vec2::ZERO).with_persistence(false),
            sleeping_disabled: SleepingDisabled,
        }
    }
}

fn spawn_player_spaceship(mut commands: Commands, assets: Res<ImageAssets>) {
    let spaceship = commands
        .spawn((Player, SpaceshipBundle { ..default() }))
        .id();

    let main_drive = commands
        .spawn(ThrusterBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -22.5, 0.0),
                rotation: Quat::from_rotation_z(std::f32::consts::PI),
                ..default()
            },
            thruster_roles: ThrusterRoles::MainDrive,
            thrust: Thrust { value: 30000.0 },
            ..Default::default()
        })
        .id();

    let thruster_NE = commands
        .spawn(ThrusterBundle {
            transform: Transform {
                translation: Vec3::new(7.5, 12.5, 0.0),
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_4 * -1.0),
                ..Default::default()
            },
            thruster_roles: ThrusterRoles::Backward
                | ThrusterRoles::Left
                | ThrusterRoles::AntiClockwise,
            ..Default::default()
        })
        .id();

    let thruster_NW = commands
        .spawn(ThrusterBundle {
            transform: Transform {
                translation: Vec3::new(-7.5, 12.5, 0.0),
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_4 * 1.0),
                ..Default::default()
            },
            thruster_roles: ThrusterRoles::Backward
                | ThrusterRoles::Right
                | ThrusterRoles::Clockwise,
            ..Default::default()
        })
        .id();

    let thruster_SW = commands
        .spawn(ThrusterBundle {
            transform: Transform {
                translation: Vec3::new(-7.5, -12.5, 0.0),
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_4 * 3.0),
                ..Default::default()
            },
            thruster_roles: ThrusterRoles::Forward
                | ThrusterRoles::Right
                | ThrusterRoles::AntiClockwise,
            ..Default::default()
        })
        .id();

    let thruster_SE = commands
        .spawn(ThrusterBundle {
            transform: Transform {
                translation: Vec3::new(7.5, -12.5, 0.0),
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_4 * -3.0),
                ..Default::default()
            },
            thruster_roles: ThrusterRoles::Forward | ThrusterRoles::Left | ThrusterRoles::Clockwise,
            ..Default::default()
        })
        .id();

    commands.entity(spaceship).add_children(&[
        main_drive,
        thruster_NE,
        thruster_NW,
        thruster_SW,
        thruster_SE,
    ]);
}

fn spaceship_control(
    mut q_spaceship: Query<&Children, (With<Player>, With<SpaceShip>)>,
    mut q_thrusters: Query<&Status>,
    mut input_event_reader: EventReader<PlayerInputEvents>,
) {
    let Ok(thrusters) = q_spaceship.get_single_mut() else {
        return;
    };

    let apply_thrust =
        |role: &ThrusterRoles, external_force: &mut ExternalForce, center_of_mass: &Transform| {
            for &child in children.iter() {
                if let Ok((thruster_roles, thruster_transform, thruster_thrust, fuel_type)) =
                    q_thrusters.get(child)
                {
                    if thruster_roles.contains(*role) {
                        let force = -thruster_transform.rotation().mul_vec3(Vec3::Y).truncate()
                            * thruster_thrust.value;
                        external_force.apply_force_at_point(
                            force,
                            thruster_transform.translation().truncate(),
                            center_of_mass.translation.truncate(),
                        );
                    }
                }
            }
        };

    for event in input_event_reader.read() {
        match event {
            PlayerInputEvents::Up => {
                apply_thrust(
                    &ThrusterRoles::Forward,
                    &mut spaceship_external_force,
                    &center_of_mass,
                );
            }
            PlayerInputEvents::Left => {
                apply_thrust(
                    &ThrusterRoles::Left,
                    &mut spaceship_external_force,
                    &center_of_mass,
                );
            }
            PlayerInputEvents::Down => {
                apply_thrust(
                    &ThrusterRoles::Backward,
                    &mut spaceship_external_force,
                    &center_of_mass,
                );
            }
            PlayerInputEvents::Right => {
                apply_thrust(
                    &ThrusterRoles::Right,
                    &mut spaceship_external_force,
                    &center_of_mass,
                );
            }
            PlayerInputEvents::MainDrive => {
                apply_thrust(
                    &ThrusterRoles::MainDrive,
                    &mut spaceship_external_force,
                    &center_of_mass,
                );
            }
            PlayerInputEvents::RotateClockwise => {
                apply_thrust(
                    &ThrusterRoles::Clockwise,
                    &mut spaceship_external_force,
                    &center_of_mass,
                );
            }
            PlayerInputEvents::RotateAntiClockwise => {
                apply_thrust(
                    &ThrusterRoles::AntiClockwise,
                    &mut spaceship_external_force,
                    &center_of_mass,
                );
            }
            // PlayerInputEventss::FireMissile =>
            // PlayerInputEventss::FirePdc =>
            // PlayerInputEventss::ToggleAutotrack =>
            // PlayerInputEventss::FireRailgun =>
            _ => info!("No event found"),
        }
    }
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player_spaceship)
            .add_systems(FixedUpdate, spaceship_control.in_set(InGameSet::Physics));
    }
}
