use avian2d::prelude::RigidBodyQueryReadOnly;
use bevy::prelude::*;

use crate::schedule::InGameSet;
use crate::spaceship::PlayerControlled;

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
    mut camera: Query<
        (&mut Transform, &mut Velocity),
        (With<MainCamera>, Without<PlayerControlled>),
    >,
    spaceship: Query<&Transform, (With<PlayerControlled>, Without<MainCamera>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let (mut cam_pos, mut cam_vel) = camera;

    let Ok(spaceship) = spaceship.get_single() else {
        return;
    };

    let spring_k = 5.1;
    let damping = 3.0;
    let dt = time.delta_secs();

    let camera_position = cam_pos.translation.truncate();
    let spaceship_position = Vec2::new(spaceship.position.x, spaceship.position.y);
    let spaceship_velocity = Vec2::new(spaceship.linear_velocity.x, spaceship.linear_velocity.y);

    let displacement = spaceship_position - camera_position;
    let spring_force = displacement * spring_k;

    let relative_vel = spaceship_velocity - cam_vel.value;
    let damping_force = relative_vel * damping;

    let total_force = spring_force + damping_force;
    cam_vel.value += total_force * dt;

    let new_position = camera_position + cam_vel.value * dt;
    cam_pos.translation = Vec3::new(new_position.x, new_position.y, cam_pos.translation.z);
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, update_camera_position.in_set(InGameSet::Render));
    }
}
