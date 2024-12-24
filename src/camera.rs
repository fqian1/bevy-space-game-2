use avian2d::prelude::{PhysicsSet, RigidBodyQueryReadOnly};
use bevy::prelude::*;

use crate::schedule::InGameSet;
use crate::spaceship::{Player, SpaceShip};

#[derive(Component, Debug)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2d));
}

fn update_camera_position(
    mut camera: Query<&mut Transform, With<MainCamera>>,
    spaceship: Query<RigidBodyQueryReadOnly, (With<SpaceShip>, With<Player>)>,
) {
    let Ok(mut camera_transform) = camera.get_single_mut() else {
        return;
    };
    let Ok(spaceship) = spaceship.get_single() else {
        return;
    };
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
