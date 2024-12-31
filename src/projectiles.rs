use avian2d::prelude::*;
use bevy::prelude::*;

use crate::durability::Durability;
use crate::thrusters::*;

#[derive(Component, Debug)]
pub struct Bullet;

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Payload(Option<f32>);

#[derive(Bundle)]
pub struct BulletBundle {
    pub marker: Bullet,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub mass: Mass,
    pub transform: Transform,
    pub external_impulse: ExternalImpulse,
    pub restitution: Restitution,
    pub payload: Payload,
}

impl Default for BulletBundle {
    fn default() -> Self {
        Self {
            marker: Bullet,
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

#[derive(Component, Debug)]
pub struct Torpedo;
#[derive(Bundle)]
pub struct TorpedoBundle {
    pub marker: Torpedo,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub mass: Mass,
    pub payload: Payload,
    pub transform: Transform,
    pub external_force: ExternalForce,
    pub restitution: Restitution,
}

impl Default for TorpedoBundle {
    fn default() -> Self {
        Self {
            marker: Torpedo,
            rigid_body: RigidBody::Dynamic,
            collider: Collider::rectangle(4.0, 8.0),
            mass: Mass(1.0),
            payload: Payload(None),
            transform: Transform::default(),
            external_force: ExternalForce::new(Vec2::ZERO).with_persistence(false),
            restitution: Restitution::new(0.8),
        }
    }
}

#[derive(Component, Debug)]
pub struct Slug;

#[derive(Bundle)]
pub struct SlugBundle {
    pub marker: Slug,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub mass: Mass,
    pub transform: Transform,
    pub external_impulse: ExternalImpulse,
    pub restitution: Restitution,
}

impl Default for SlugBundle {
    fn default() -> Self {
        Self {
            marker: Slug,
            rigid_body: RigidBody::Dynamic,
            collider: Collider::rectangle(5.0, 20.0),
            mass: Mass(9.0),
            transform: Transform::default(),
            external_impulse: ExternalImpulse::new(Vec2::ZERO),
            restitution: Restitution::new(0.3),
        }
    }
}
