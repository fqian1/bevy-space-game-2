use std::fmt::Debug;

use avian2d::prelude::*;
use bevy::{
    // core::FrameCount,
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{CursorGrabMode, PresentMode, SystemCursorIcon, WindowLevel, WindowTheme},
    // winit::cursor::CursorIcon,
};
use bevy_hanabi::prelude::*;

mod asset_loader;
mod camera;
mod cleanup;
mod collision;
mod debug;
mod durability;
mod fuel;
mod game_state;
mod projectiles;
mod schedule;
mod spaceship;
mod system_status;
mod temperature;
mod thrusters;
mod weapons;

use asset_loader::AssetLoaderPlugin;
use camera::CameraPlugin;
// use cleanup::CleanupPlugin;
// use collision::CollisionPlugin;
use debug::DebugPlugin;
// use projectiles::ProjectilesPlugin;
use game_state::GameStatePlugin;
use schedule::SchedulePlugin;
use spaceship::SpaceshipPlugin;
// use weapons::WeaponsPlugin;
use durability::DurabilityPlugin;
use thrusters::ThrusterPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Space Game".to_string(),
                    name: Some("Space Game".to_string()),
                    resolution: (1024.0, 720.0).into(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..default()
                    },
                    // present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
            // LogDiagnosticsPlugin::default(),
            // FrameTimeDiagnosticsPlugin,
            PhysicsPlugins::default().with_length_unit(100.0),
            AssetLoaderPlugin,
            CameraPlugin,
            // CleanupPlugin,
            // CollisionPlugin,
            // ControllerPlugin,
            DebugPlugin,
            // HealthPlugin,
            // ProjectilesPlugin,
            SchedulePlugin,
            GameStatePlugin,
            SpaceshipPlugin,
            // WeaponsPlugin,
            DurabilityPlugin,
            HanabiPlugin,
            ThrusterPlugin,
        ))
        .insert_resource(Gravity(Vec2::ZERO))
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .run();
}
