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
        Sprite::from_image(assets.spaceship.clone()),
    ));
}

fn spawn_player_spaceship(mut commands: Commands, assets: Res<ImageAssets>) {
    commands.spawn((
        PlayerControlled,
        SpaceShip,
        RigidBody::Dynamic,
        Collider::rectangle(50.0, 100.0),
        SleepingDisabled,
        Transform::from_translation(Vec3::ZERO),
        Sprite::from_image(assets.spaceship.clone()),
    ));
}

fn spaceship_movement_control(
    mut query: Query<
        (&mut Transform, &mut LinearVelocity, &mut AngularVelocity),
        With<PlayerControlled>,
    >,
    mut input_event_reader: EventReader<SpaceshipControlEvents>,
) {
    let Ok((transform, mut linear_velocity, mut angular_velocity)) = query.get_single_mut() else {
        return;
    };
    let forward = transform.rotation.mul_vec3(Vec3::Y).truncate();
    let right = transform.rotation.mul_vec3(Vec3::X).truncate();
    for event in input_event_reader.read() {
        match event {
            SpaceshipControlEvents::ThrustForward => {
                linear_velocity.x += forward.x;
                linear_velocity.y += forward.y;
            }
            SpaceshipControlEvents::ThrustLeft => {
                linear_velocity.x += -right.x;
                linear_velocity.y += -right.y;
            }
            SpaceshipControlEvents::ThrustBackward => {
                linear_velocity.x += -forward.x;
                linear_velocity.y += -forward.y;
            }
            SpaceshipControlEvents::ThrustRight => {
                linear_velocity.x += right.x;
                linear_velocity.y += right.y;
            }
            SpaceshipControlEvents::MainDrive => {
                linear_velocity.x += forward.x * 2.0;
                linear_velocity.y += forward.y * 2.0;
            }
            SpaceshipControlEvents::ThrustClockwise => angular_velocity.0 -= 0.1,
            SpaceshipControlEvents::ThrustAntiClockwise => angular_velocity.0 += 0.1,
            _ => info!("uh"),
            // SpaceshipControlEvents::ThrustClockwise =>
            // SpaceshipControlEvents::ThrustAntiClockwise =>
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
