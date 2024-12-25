use bevy::prelude::*;

#[derive(Component, Debug, Default, Clone, Copy)]
pub enum SystemStatus {
    Active,
    #[default]
    Ready,
    Vulnerable,
    Broken,
    Repairing,
}
