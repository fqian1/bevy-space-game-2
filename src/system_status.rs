use bevy::prelude::*;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SystemStatus {
    Active,
    #[default]
    Ready,
    Vulnerable,
    Broken,
    Repairing,
    Loading,
}
