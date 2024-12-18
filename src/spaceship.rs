use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

use crate::asset_loader::ImageAssets;
use crate::controller::SpaceshipControlEvents;
use crate::schedule::InGameSet;
use crate::state::GameState;

#[derive(Component, Debug)]
pub struct SpaceShip;

#[derive(Component)]
pub struct PlayerControlled;

#[derive(Component)]
pub struct AiControlled;

fn spawn_spaceship(mut commands: Commands, assets: Res<ImageAssets>) {
    commands.spawn((
        AiControlled,
        SpaceShip,
        RigidBody::Dynamic,
        Collider::rectangle(50.0, 100.0),
        Transform::from_translation(Vec3::ZERO),
        Sprite::from_image(assets.ship_base_full_health.clone()),
    ));
}

fn spawn_player_spaceship(mut commands: Commands, assets: Res<ImageAssets>) {
    commands.spawn((
        PlayerControlled,
        SpaceShip,
        RigidBody::Dynamic,
        Collider::rectangle(50.0, 100.0),
        SleepingDisabled,
        ExternalForce::new(Vec2::ZERO).with_persistence(false),
        Transform::from_translation(Vec3::ZERO),
        Sprite::from_image(assets.ship_base_full_health.clone()),
    ));
}

fn spaceship_movement_control(
    mut query: Query<(&mut Transform, &mut ExternalForce), With<PlayerControlled>>,
    mut input_event_reader: EventReader<SpaceshipControlEvents>,
) {
    let Ok((transform, mut external_force)) = query.get_single_mut() else {
        return;
    };
    let forward = transform.rotation.mul_vec3(Vec3::Y).truncate();
    let right = transform.rotation.mul_vec3(Vec3::X).truncate();
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
