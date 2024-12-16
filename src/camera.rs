use bevy::prelude::*;

use crate::schedule::InGameSet;
use crate::spaceship::PlayerControlled;

#[derive(Component, Debug)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2d::default()));
}

// fn update_camera_position(mut camera: Query<&mut Transform, (With<Camera2d>, Without<PlayerControlled>)>,
//     spaceship: Query<&Transform,
//     (With<PlayerControlled>, Without<Camera2d>)>,
//     time: Res<Time>)
// {
//     let Ok(mut camera) = camera.get_single_mut() else {
//         return;

//     };

//     let Ok(spaceship) = spaceship.get_single() else {
//         return;
//     };

//     let Vec3 { x, y, .. } = spaceship.translation;
//     let direction = Vec3::new(x, y, camera.translation.z);

//     camera.translation.smooth_nudge(&direction, 2.0, time.delta_secs());
//     camera.rotation = spaceship.rotation;
// }

fn update_camera_position(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<PlayerControlled>)>,
    spaceship: Query<&Transform, (With<PlayerControlled>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };
    let Ok(spaceship) = spaceship.get_single() else {
        return;
    };

    let target = Vec3::new(
        spaceship.translation.x,
        spaceship.translation.y,
        camera.translation.z,
    );

    camera.translation = camera.translation.lerp(target, time.delta_secs() * 2.0);
    // Remove rotation copying
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, update_camera_position.in_set(InGameSet::Render));
    }
}
