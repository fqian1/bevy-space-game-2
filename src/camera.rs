use avian2d::prelude::*;
use bevy::prelude::*;

use crate::schedule::InGameSet;
use crate::spaceship::{Player, SpaceShip};

#[derive(Component, Debug)]
pub struct CameraMarker;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((CameraMarker, Camera2d));
}

fn update_camera_position(
    mut camera: Query<&mut Transform, (With<CameraMarker>, Without<Player>)>,
    spaceship: Query<(&Transform, &ExternalForce), (With<SpaceShip>, With<Player>)>,
    time: Res<Time>,
) {
    let Ok(mut camera_transform) = camera.get_single_mut() else {
        return;
    };
    let Ok((spaceship_transform, spaceship_external_force)) = spaceship.get_single() else {
        return;
    };
    // let lag_factor = 0.0001;
    // info!("spaceship force: {:?}", spaceship_external_force.force());
    // let camera_target = (spaceship_transform.translation.truncate()
    //     - spaceship_external_force.force())
    //     * lag_factor;
    // camera_transform.translation.smooth_nudge(
    //     &spaceship_transform.translation,
    //     9.0,
    //     time.delta_secs(),
    // );
    camera_transform.translation = spaceship_transform.translation;
    camera_transform.rotation = spaceship_transform.rotation;
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera).add_systems(
            FixedUpdate,
            update_camera_position.in_set(InGameSet::Render),
        );
    }
}
