use bevy::prelude::*;

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Health {
    #[deref]
    pub value: f32,
}
