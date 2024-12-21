use bevy::prelude::*;
// use bevy_enhanced_input::prelude::*;

use crate::schedule::InGameSet;

#[derive(Event, Debug)]
pub enum PlayerInputEvents {
    Up,
    Down,
    Left,
    Right,
    MainDrive,
    RotateClockwise,
    RotateAntiClockwise,
    FireMissile,
    FirePdc,
    ToggleAutotrack,
    FireRailgun,
}

fn write_player_input_events(
    mut input_event_writer: EventWriter<PlayerInputEvents>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::KeyW) {
        input_event_writer.send(PlayerInputEvents::Up);
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        input_event_writer.send(PlayerInputEvents::Left);
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        input_event_writer.send(PlayerInputEvents::Down);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        input_event_writer.send(PlayerInputEvents::Right);
    }
    if keyboard_input.pressed(KeyCode::Space) {
        input_event_writer.send(PlayerInputEvents::MainDrive);
    }
    if keyboard_input.pressed(KeyCode::KeyE) {
        input_event_writer.send(PlayerInputEvents::RotateClockwise);
    }
    if keyboard_input.pressed(KeyCode::KeyQ) {
        input_event_writer.send(PlayerInputEvents::RotateAntiClockwise);
    }
    if keyboard_input.pressed(KeyCode::KeyF) {
        input_event_writer.send(PlayerInputEvents::FirePdc);
    }
}

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputEvents>().add_systems(
            FixedUpdate,
            write_player_input_events.in_set(InGameSet::Input),
        );
    }
}
