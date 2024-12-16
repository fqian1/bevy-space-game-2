use bevy::prelude::*;

use crate::schedule::InGameSet;

#[derive(Component, Debug)]
pub struct PlayerController;

#[derive(Component, Debug)]
pub struct AIController;

#[derive(Event, Debug)]
pub enum SpaceshipControlEvents {
    ThrustForward,
    ThrustLeft,
    ThrustBackward,
    ThrustRight,
    MainDrive,
    ThrustClockwise,
    ThrustAntiClockwise,
    FireMissile,
    FirePdc,
    ToggleAutotrack,
    FireRailgun,
}

fn write_player_input_events(mut input_event_writer: EventWriter<SpaceshipControlEvents>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::KeyW) {
        input_event_writer.send(SpaceshipControlEvents::ThrustForward);
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        input_event_writer.send(SpaceshipControlEvents::ThrustLeft);
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        input_event_writer.send(SpaceshipControlEvents::ThrustBackward);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        input_event_writer.send(SpaceshipControlEvents::ThrustRight);
    }
    if keyboard_input.pressed(KeyCode::Space) {
        input_event_writer.send(SpaceshipControlEvents::MainDrive);
    }
    if keyboard_input.pressed(KeyCode::KeyE) {
        input_event_writer.send(SpaceshipControlEvents::ThrustClockwise);
    }
    if keyboard_input.pressed(KeyCode::KeyQ) {
        input_event_writer.send(SpaceshipControlEvents::ThrustAntiClockwise);
    }
    if keyboard_input.pressed(KeyCode::KeyF) {
        input_event_writer.send(SpaceshipControlEvents::FirePdc);
    }
}

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin{
    fn build(&self, app: &mut App) {
        app.add_event::<SpaceshipControlEvents>()
        .add_systems(Update, write_player_input_events.in_set(InGameSet::Input));
    }
}