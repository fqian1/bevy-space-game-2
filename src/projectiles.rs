use avian2d::prelude::*;
use bevy::prelude::*;

use crate::durability::Durability;
use crate::thrusters::*;

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Payload(Option<f32>);

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub mass: Mass,
    pub transform: Transform,
    pub external_impulse: ExternalImpulse,
    pub restitution: Restitution,
    pub payload: Payload,
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::rectangle(2.0, 4.0),
            mass: Mass(0.1),
            transform: Transform::default(),
            external_impulse: ExternalImpulse::new(Vec2::Y * 100.0),
            restitution: Restitution::new(0.8),
            payload: Payload(None),
        }
    }
}
