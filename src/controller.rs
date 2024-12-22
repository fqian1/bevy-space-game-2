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
    None,
}

fn write_player_input_events(
    mut input_event_writer: EventWriter<PlayerInputEvents>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // if keyboard_input.pressed(KeyCode::KeyW) {
    //     input_event_writer.send(PlayerInputEvents::Up);
    // }
    // if keyboard_input.pressed(KeyCode::KeyA) {
    //     input_event_writer.send(PlayerInputEvents::Left);
    // }
    // if keyboard_input.pressed(KeyCode::KeyS) {
    //     input_event_writer.send(PlayerInputEvents::Down);
    // }
    // if keyboard_input.pressed(KeyCode::KeyD) {
    //     input_event_writer.send(PlayerInputEvents::Right);
    // }
    // if keyboard_input.pressed(KeyCode::Space) {
    //     input_event_writer.send(PlayerInputEvents::MainDrive);
    // }
    // if keyboard_input.pressed(KeyCode::KeyE) {
    //     input_event_writer.send(PlayerInputEvents::RotateClockwise);
    // }
    // if keyboard_input.pressed(KeyCode::KeyQ) {
    //     input_event_writer.send(PlayerInputEvents::RotateAntiClockwise);
    // }
    // if keyboard_input.pressed(KeyCode::KeyF) {
    //     input_event_writer.send(PlayerInputEvents::FirePdc);
    // }
    // if keyboard_input.pressed(KeyCode::KeyR) {
    //     input_event_writer.send(PlayerInputEvents::FireMissile);
    // }
    // if keyboard_input.pressed(KeyCode::KeyT) {
    //     input_event_writer.send(PlayerInputEvents::ToggleAutotrack);
    // }
    // if keyboard_input.pressed(KeyCode::KeyG) {
    //     input_event_writer.send(PlayerInputEvents::FireRailgun);
    // }

    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::KeyW => input_event_writer.send(PlayerInputEvents::Up),
            KeyCode::KeyA => input_event_writer.send(PlayerInputEvents::Left),
            KeyCode::KeyS => input_event_writer.send(PlayerInputEvents::Down),
            KeyCode::KeyD => input_event_writer.send(PlayerInputEvents::Right),
            KeyCode::Space => input_event_writer.send(PlayerInputEvents::MainDrive),
            KeyCode::KeyE => input_event_writer.send(PlayerInputEvents::RotateClockwise),
            KeyCode::KeyQ => input_event_writer.send(PlayerInputEvents::RotateAntiClockwise),
            KeyCode::KeyF => input_event_writer.send(PlayerInputEvents::FirePdc),
            KeyCode::KeyR => input_event_writer.send(PlayerInputEvents::FireMissile),
            KeyCode::KeyT => input_event_writer.send(PlayerInputEvents::ToggleAutotrack),
            KeyCode::KeyG => input_event_writer.send(PlayerInputEvents::FireRailgun),
            _ => input_event_writer.send(PlayerInputEvents::None),
        };
    }

    for key in keyboard_input.get_just_released() {
        match key {
            KeyCode::KeyW => input_event_writer.send(PlayerInputEvents::None),
            KeyCode::KeyA => input_event_writer.send(PlayerInputEvents::None),
            KeyCode::KeyS => input_event_writer.send(PlayerInputEvents::None),
            KeyCode::KeyD => input_event_writer.send(PlayerInputEvents::None),
            KeyCode::Space => input_event_writer.send(PlayerInputEvents::None),
            KeyCode::KeyE => input_event_writer.send(PlayerInputEvents::None),
            KeyCode::KeyQ => input_event_writer.send(PlayerInputEvents::None),
            KeyCode::KeyF => input_event_writer.send(PlayerInputEvents::None),
            KeyCode::KeyR => input_event_writer.send(PlayerInputEvents::None),
            KeyCode::KeyT => input_event_writer.send(PlayerInputEvents::None),
            KeyCode::KeyG => input_event_writer.send(PlayerInputEvents::None),
            _ => input_event_writer.send(PlayerInputEvents::None),
        };
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
