use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::asset_loader::ImageAssets;
use crate::controller::PlayerInputEvents;
use crate::schedule::InGameSet;
use crate::state::GameState;
use crate::thrusters::{Status, Thruster, ThrusterBundle, ThrusterRoles};
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
        Self {
            spaceship: SpaceShip,
            rigid_body: RigidBody::Dynamic,
            collider: Collider::rectangle(15.0, 25.0),
            transform: Transform::from_translation(Vec3::ZERO),
            external_force: ExternalForce::new(Vec2::ZERO).with_persistence(false),
            sleeping_disabled: SleepingDisabled,
        }
    }
}

fn spawn_player_spaceship(mut commands: Commands, assets: Res<ImageAssets>) {
    let spaceship = commands
        .spawn((
            Player,
            Sprite::from_image(assets.ship_base_full_health.clone()),
            SpaceshipBundle { ..default() },
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

    let thrusters: Vec<Entity> = (0..5)
        .map(|i| {
            commands
                .spawn((
                    ThrusterBundle {
                        transform: Transform::from_translation(thruster_locations[i].extend(0.0))
                            .with_rotation(Quat::from_rotation_z(thruster_rotations[i])),
                        roles: thruster_roles[i],
                        ..default()
                    },
                    Sprite::from_image(assets.ship_engine_base.clone()),
                ))
                .id()
        })
        .collect();

    commands.entity(spaceship).add_children(&thrusters);
}

fn spaceship_control(
    q_spaceship: Query<&Children, (With<Player>, With<SpaceShip>)>,
    mut q_thrusters: Query<(&mut Status, &ThrusterRoles)>,
    mut input_event_reader: EventReader<PlayerInputEvents>,
) {
    let Ok(thrusters) = q_spaceship.get_single() else {
        return;
    };

    let mut set_status = |role: &ThrusterRoles| {
        for &thruster in thrusters.iter() {
            let Ok((mut status, thruster_role)) = q_thrusters.get_mut(thruster) else {
                return;
            };
            if thruster_role.contains(*role) {
                *status = Status::Active;
            } else {
                *status = Status::Inactive;
            }
        }
    };

    for event in input_event_reader.read() {
        match event {
            PlayerInputEvents::Up => {
                set_status(&ThrusterRoles::Forward);
            }
            PlayerInputEvents::Left => {
                set_status(&ThrusterRoles::Left);
            }
            PlayerInputEvents::Down => {
                set_status(&ThrusterRoles::Backward);
            }
            PlayerInputEvents::Right => {
                set_status(&ThrusterRoles::Right);
            }
            PlayerInputEvents::MainDrive => {
                set_status(&ThrusterRoles::MainDrive);
            }
            PlayerInputEvents::RotateClockwise => {
                set_status(&ThrusterRoles::Clockwise);
            }
            PlayerInputEvents::RotateAntiClockwise => {
                set_status(&ThrusterRoles::AntiClockwise);
            }
            // PlayerInputEventss::FireMissile =>
            // PlayerInputEventss::FirePdc =>
            // PlayerInputEventss::ToggleAutotrack =>
            // PlayerInputEventss::FireRailgun =>
            _ => set_status(&ThrusterRoles::None),
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
