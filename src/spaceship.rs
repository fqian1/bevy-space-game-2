use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

use crate::asset_loader::ImageAssets;
use crate::controller::SpaceshipControlEvents;
use crate::schedule::InGameSet;
use crate::state::GameState;
use crate::thrusters::*;
use crate::weapons::*;

#[derive(Component, Debug)]
pub struct SpaceShip;

#[derive(Component)]
pub struct PlayerControlled;

#[derive(Component)]
pub struct AiControlled;

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
    pub sprite: Sprite,
    pub external_force: ExternalForce,
    pub sleeping_disabled: SleepingDisabled,
}

impl Default for SpaceshipBundle {
    fn default() -> Self {
        Self {
            spaceship: SpaceShip,
            rigid_body: RigidBody::Dynamic,
            collider: Collider::rectangle(10.0, 10.0),
            transform: Transform::from_translation(Vec3::ZERO),
            sprite: Sprite::from_image(ImageAssets::default().ship_base_full_health),
            external_force: ExternalForce::new(Vec2::ZERO).with_persistence(false),
            sleeping_disabled: SleepingDisabled,
        }
    }
}

fn spawn_player_spaceship(mut commands: Commands, assets: Res<ImageAssets>) {
    let spaceship = commands
        .spawn((PlayerControlled, SpaceshipBundle {
            sprite: Sprite::from_image(assets.ship_base_full_health.clone()),
            ..Default::default()
        }))
        .id();

    let main_drive = commands
        .spawn(ThrusterBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -5.0, 0.0),
                rotation: Quat::from_rotation_z(std::f32::consts::PI),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let thruster_NE = commands
        .spawn(ThrusterBundle {
            transform: Transform {
                translation: Vec3::new(5.0, 5.0, 0.0),
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_4),
                ..Default::default()
            },
            thruster_roles: ThrusterRoles::Backward | ThrusterRoles::Left,
            ..Default::default()
        })
        .id();

    let thruster_NW = commands
        .spawn(ThrusterBundle {
            transform: Transform {
                translation: Vec3::new(-5.0, 5.0, 0.0),
                rotation: Quat::from_rotation_z(-std::f32::consts::FRAC_PI_4),
                ..Default::default()
            },
            thruster_roles: ThrusterRoles::Backward | ThrusterRoles::Right,
            ..Default::default()
        })
        .id();

    let thruster_SW = commands
        .spawn(ThrusterBundle {
            transform: Transform {
                translation: Vec3::new(5.0, -5.0, 0.0),
                rotation: Quat::from_rotation_z(-std::f32::consts::FRAC_PI_4 * 3.0),
                ..Default::default()
            },
            thruster_roles: ThrusterRoles::Forward | ThrusterRoles::Right,
            ..Default::default()
        })
        .id();

    let thruster_SE = commands
        .spawn(ThrusterBundle {
            transform: Transform {
                translation: Vec3::new(-5.0, -5.0, 0.0),
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_4 * 3.0),
                ..Default::default()
            },
            thruster_roles: ThrusterRoles::Forward | ThrusterRoles::Left,
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

fn spaceship_movement_control(
    mut q_spaceship: Query<(&Children, &mut ExternalForce), With<PlayerControlled>>,
    mut q_thrusters: Query<(&ThrusterRoles, &Transform, &Thrust, &FuelType)>,
    mut input_event_reader: EventReader<SpaceshipControlEvents>,
) {
    let Ok((children, mut spaceship_external_force)) = q_spaceship.get_single_mut() else {
        return;
    };

    let apply_thrust = |role: &ThrusterRoles, external_force: &mut ExternalForce| {
        for &child in children.iter() {
            if let Ok((thruster_roles, thruster_transform, thruster_thrust, fuel_type)) =
                q_thrusters.get(child)
            {
                if thruster_roles.contains(*role) {
                    let force = if role.contains(ThrusterRoles::Forward)
                        || role.contains(ThrusterRoles::MainDrive)
                        || role.contains(ThrusterRoles::Backward)
                    {
                        -thruster_transform.rotation.mul_vec3(Vec3::Y).truncate()
                    } else {
                        thruster_transform.rotation.mul_vec3(Vec3::Y).truncate()
                    } * thruster_thrust.value;

                    external_force.apply_force(force);
                }
            }
        }
    };

    for event in input_event_reader.read() {
        match event {
            SpaceshipControlEvents::ThrustForward => {
                apply_thrust(&ThrusterRoles::Forward, &mut spaceship_external_force);
            }
            SpaceshipControlEvents::ThrustLeft => {
                apply_thrust(&ThrusterRoles::Left, &mut spaceship_external_force);
            }
            SpaceshipControlEvents::ThrustBackward => {
                apply_thrust(&ThrusterRoles::Backward, &mut spaceship_external_force);
            }
            SpaceshipControlEvents::ThrustRight => {
                apply_thrust(&ThrusterRoles::Right, &mut spaceship_external_force);
            }
            SpaceshipControlEvents::MainDrive => {
                apply_thrust(&ThrusterRoles::MainDrive, &mut spaceship_external_force);
            }
            // SpaceshipControlEvents::ThrustClockwise => angular_velocity.0 -= 0.1,
            // SpaceshipControlEvents::ThrustAntiClockwise => angular_velocity.0 += 0.1,
            // SpaceshipControlEvents::FireMissile =>
            // SpaceshipControlEvents::FirePdc =>
            // SpaceshipControlEvents::ToggleAutotrack =>
            // SpaceshipControlEvents::FireRailgun =>
            _ => info!("No event found"),
        }
    }
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player_spaceship)
            .add_systems(
                FixedUpdate,
                spaceship_movement_control.in_set(InGameSet::Physics),
            );
    }
}
