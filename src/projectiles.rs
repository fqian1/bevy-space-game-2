use avian2d::prelude::*;
use bevy::prelude::*;

use crate::thrusters::*;

#[derive(Component, Debug)]
pub struct Bullet;

#[derive(Bundle)]
pub struct BulletBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub mass: Mass,
    pub transform: Transform,
    pub external_force: ExternalForce,
    pub restitution: Restitution,
}

impl Default for BulletBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::rectangle(2.0, 4.0),
            mass: Mass(1.0),
            transform: Transform::default(),
            external_force: ExternalForce::new(Vec2::ZERO).with_persistence(false),
            restitution: Restitution::new(0.8),
        }
    }
}

#[derive(Component, Debug)]
pub struct Torpedo;

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Payload(pub f32);

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct FuelCapacity(pub f32);

#[derive(Bundle)]
pub struct TorpedoBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub mass: Mass,
    pub transform: Transform,
    pub external_force: ExternalForce,
    pub restitution: Restitution,
}

impl Default for TorpedoBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::rectangle(4.0, 8.0),
            mass: Mass(1.0),
            transform: Transform::default(),
            external_force: ExternalForce::new(Vec2::ZERO).with_persistence(false),
            restitution: Restitution::new(0.8),
        }
    }
}
