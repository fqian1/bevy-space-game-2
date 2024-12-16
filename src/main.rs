use std::fmt::Debug;

use avian2d::prelude::*;
use bevy::prelude::*;

mod asset_loader;
mod camera;
mod cleanup;
mod collision;
mod controller;
mod debug;
mod health;
mod projectiles;
mod schedule;
mod spaceship;
mod state;
mod weapons;

use asset_loader::AssetLoaderPlugin;
use camera::CameraPlugin;
// use cleanup::CleanupPlugin;
// use collision::CollisionPlugin;
use controller::ControllerPlugin;
use debug::DebugPlugin;
// use health::HealthPlugin;
// use projectiles::ProjectilesPlugin;
use schedule::SchedulePlugin;
use spaceship::SpaceshipPlugin;
use state::StatePlugin;
// use weapons::WeaponsPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_length_unit(100.0),
        ))
        .insert_resource(Gravity(Vec2::ZERO))
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        // .add_plugins(CleanupPlugin)
        // .add_plugins(CollisionPlugin)
        .add_plugins(ControllerPlugin)
        .add_plugins(DebugPlugin)
        // .add_plugins(HealthPlugin)
        // .add_plugins(ProjectilesPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .add_plugins(SpaceshipPlugin)
        // .add_plugins(WeaponsPlugin)
        .run();
}
