use avian2d::prelude::{PhysicsSet, RigidBodyQueryReadOnly};
use bevy::prelude::*;

use crate::schedule::InGameSet;
use crate::spaceship::PlayerControlled;

const SPRING_CONSTANT: f32 = 1.05;
const DAMPING_FACTOR: f32 = 1.0;

#[derive(Component, Debug)]
pub struct MainCamera;

#[derive(Component)]
pub struct Velocity {
    value: Vec2,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { value: Vec2::ZERO }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2d, Velocity::default()));
}

fn update_camera_position(
    mut camera: Query<(&mut Transform, &mut Velocity), With<MainCamera>>,
    spaceship: Query<RigidBodyQueryReadOnly, With<PlayerControlled>>,
    time: Res<Time>,
) {
    let Ok((mut camera_transform, mut camera_velocity)) = camera.get_single_mut() else {
        return;
    };
    let Ok(spaceship) = spaceship.get_single() else {
        return;
    };

    // let delta_time = time.delta_secs();

    // let camera_pos = camera_transform.translation.truncate();
    // let ship_pos = Vec2::new(spaceship.position.x, spaceship.position.y);
    // let ship_velocity = Vec2::new(spaceship.linear_velocity.x, spaceship.linear_velocity.y);

    // let displacement = ship_pos - camera_pos;
    // let spring_force = displacement * SPRING_CONSTANT;

    // let relative_velocity = ship_velocity - camera_velocity.value;
    // let damping_force = relative_velocity * DAMPING_FACTOR;

    // let total_force = spring_force + damping_force;
    // camera_velocity.value += total_force * delta_time;

    // let new_position = camera_pos + camera_velocity.value * delta_time;

    // let z = camera_transform.translation.z;

    // // camera_transform.translation = Vec3::new(
    // //     new_position.x,
    // //     new_position.y,
    // //     camera_transform.translation.z,
    // // );

    // camera_transform.translation.smooth_nudge(
    //     &Vec3::new(ship_pos.x, ship_pos.y, z),
    //     0.001,
    //     delta_time,
    // );

    // let theta = spaceship.rotation.sin.atan2(spaceship.rotation.cos);
    // camera_transform.rotation = Quat::from_rotation_z(theta);
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(FixedUpdate, update_camera_position);
    }
}
