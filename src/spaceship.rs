use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

use crate::asset_loader::ImageAssets;
use crate::controller::SpaceshipControlEvents;
use crate::schedule::InGameSet;
use crate::state::GameState;
use crate::thrusters;
use crate::weapons;

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
    commands
        .spawn((PlayerControlled, SpaceshipBundle {
            sprite: Sprite::from_image(assets.ship_base_full_health.clone()),
            ..Default::default()
        }))
        .with_children(|parent| {
            parent.spawn(thrusters::ThrusterBundle {
                sprite: Sprite::from_image(assets.ship_weapon_auto_cannon.clone()),
                transform: Transform::from_translation(Vec3::new(0.0, -5.0, 0.0)),
                thrust: thrusters::Thrust { value: 1000.0 },
                fuel_type: thrusters::FuelType::FusionPellets(1.0),
                ..Default::default()
            });
        })
        .with_children(|parent| {
            parent.spawn(thrusters::ThrusterBundle {
                sprite: Sprite::from_image(assets.ship_weapon_auto_cannon.clone()),
                transform: Transform {
                    translation: Vec3::new(0.0, -5.0, 0.0),
                    rotation: Quat::from_rotation_z(std::f32::consts::PI),
                    ..default()
                },
                thrust: thrusters::Thrust { value: 1000.0 },
                fuel_type: thrusters::FuelType::FusionPellets(1.0),
                ..Default::default()
            });
        })
        .with_children(|parent| {
            parent.spawn(thrusters::ThrusterBundle {
                sprite: Sprite::from_image(assets.ship_weapon_auto_cannon.clone()),
                transform: Transform {
                    translation: Vec3::new(5.0, -5.0, 0.0),
                    rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_4 * 3.0),
                    ..default()
                },
                thrust: thrusters::Thrust { value: 1000.0 },
                fuel_type: thrusters::FuelType::FusionPellets(1.0),
                ..Default::default()
            });
        })
        .with_children(|parent| {
            parent.spawn(thrusters::ThrusterBundle {
                sprite: Sprite::from_image(assets.ship_weapon_auto_cannon.clone()),
                transform: Transform {
                    translation: Vec3::new(-5.0, -5.0, 0.0),
                    rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_4 * 5.0),
                    ..default()
                },
                thrust: thrusters::Thrust { value: 1000.0 },
                fuel_type: thrusters::FuelType::FusionPellets(1.0),
                ..Default::default()
            });
        });
}

fn spaceship_movement_control(
    mut query: Query<(&mut Transform, &mut ExternalForce), With<PlayerControlled>>,
    mut input_event_reader: EventReader<SpaceshipControlEvents>,
) {
    let Ok((transform, mut external_force)) = query.get_single_mut() else {
        return;
    };

    for event in input_event_reader.read() {
        match event {
            SpaceshipControlEvents::ThrustForward => {
                external_force.apply_force_at_point(Vec2::Y * 1000.0, -Vec2::Y, Vec2::ZERO);
                info!("applying force: {:?}", external_force);
            }
            SpaceshipControlEvents::ThrustLeft => {
                external_force.apply_force_at_point(Vec2::X * -1000.0, -Vec2::Y, Vec2::ZERO);
            }
            SpaceshipControlEvents::ThrustBackward => {
                external_force.apply_force_at_point(Vec2::Y * -1000.0, -Vec2::Y, Vec2::ZERO);
            }
            SpaceshipControlEvents::ThrustRight => {
                external_force.apply_force_at_point(Vec2::X * 1000.0, -Vec2::Y, Vec2::ZERO);
            }
            SpaceshipControlEvents::MainDrive => {
                external_force.apply_force_at_point(Vec2::Y * 10000.0, -Vec2::Y, Vec2::ZERO);
            }
            // SpaceshipControlEvents::ThrustClockwise => angular_velocity.0 -= 0.1,
            // SpaceshipControlEvents::ThrustAntiClockwise => angular_velocity.0 += 0.1,
            _ => info!("uh"),
            // SpaceshipControlEvents::FireMissile =>
            // SpaceshipControlEvents::FirePdc =>
            // SpaceshipControlEvents::ToggleAutotrack =>
            // SpaceshipControlEvents::FireRailgun =>
        }
    }
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player_spaceship)
            .add_systems(
                Update,
                spaceship_movement_control.in_set(InGameSet::Physics),
            );
    }
}
