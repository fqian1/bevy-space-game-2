use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

use crate::asset_loader::ImageAssets;
use crate::controller::SpaceshipControlEvents;
use crate::schedule::InGameSet;
use crate::state::GameState;

#[derive(Component)]
pub struct SpaceShip;

#[derive(Component)]
pub struct PlayerControlled;

#[derive(Component)]
pub struct AiControlled;

fn spawn_spaceship(mut commands: Commands, assets: Res<ImageAssets>) {
    commands.spawn((
        SpaceShip,
        RigidBody::Dynamic,
        ExternalForce::new(Vec2::ZERO).with_persistence(false),
        Collider::rectangle(50.0, 100.0),
        ColliderDensity(2.0),
        Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(1.0)),
        Sprite::from_image(assets.spaceship.clone()),
    ));
}

fn spawn_player_spaceship(mut commands: Commands, assets: Res<ImageAssets>) {
    commands.spawn((
        PlayerControlled,
        SpaceShip,
        RigidBody::Dynamic,
        Collider::rectangle(50.0, 100.0),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.0)), // Explicitly set scale
        Sprite::from_image(assets.spaceship.clone()),
    ));
}

fn spaceship_movement_control(
    mut query: Query<(&mut Transform, &mut LinearVelocity), With<PlayerControlled>>,
    mut input_event_reader: EventReader<SpaceshipControlEvents>,
) {
    let Ok((transform, mut linear_velocity)) = query.get_single_mut() else {
        return;
    };
    for event in input_event_reader.read() {
        match event {
            SpaceshipControlEvents::ThrustForward => {
                let rotation = transform.rotation.mul_vec3(Vec3::Y).truncate();
                linear_velocity.x += rotation.x;
                linear_velocity.y += rotation.y;
            }
            _ => info!("uh"), // SpaceshipControlEvents::ThrustLeft =>
                              // SpaceshipControlEvents::ThrustBackward =>
                              // SpaceshipControlEvents::ThrustRight =>
                              // SpaceshipControlEvents::MainDrive =>
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
        app.add_systems(OnEnter(GameState::InGame), spawn_player_spaceship)
            .add_systems(
                Update,
                spaceship_movement_control.in_set(InGameSet::Physics),
            );
    }
}
