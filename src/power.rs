use bevy::prelude::*;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    #[default]
    Powered,
    Unpowered,
}
