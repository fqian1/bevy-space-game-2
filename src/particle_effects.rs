use std::sync::Once;

use bevy::prelude::*;
use bevy_hanabi::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct EffectAssets {
    pub thruster_gas: Handle<EffectAsset>,
}

pub fn setup(mut effects: ResMut<Assets<EffectAsset>>, mut effect_assets: ResMut<EffectAssets>) {
    // Create a more dynamic color gradient for the thruster
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::splat(1.0));
    color_gradient.add_key(0.3, Vec4::splat(0.8)); // Orange middle
    color_gradient.add_key(1.0, Vec4::splat(0.0)); // Fade to transparent gray

    // Create a more interesting size gradient
    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, Vec3::splat(0.05)); // Start small
    size_gradient.add_key(0.3, Vec3::splat(1.0)); // Expand quickly
    size_gradient.add_key(0.8, Vec3::splat(1.0)); // Expand quickly
    size_gradient.add_key(1.0, Vec3::splat(0.1)); // Shrink as it fades

    let writer = ExprWriter::new();

    // Initialize age and lifetime
    let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.).expr());
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        writer
            .rand(ScalarType::Float)
            .mul(writer.lit(0.02))
            .add(writer.lit(0.01))
            .expr(),
    );

    // Create a tighter cone shape for the thruster
    let init_pos = SetPositionCone3dModifier {
        base_radius: writer.lit(0.05).expr(), // Narrow start
        top_radius: writer.lit(1.1).expr(),   // Wider spread at the end
        height: writer.lit(8.0).expr(),       // Shorter length for 2D
        dimension: ShapeDimension::Volume,    // Use volume for more natural distribution
    };

    // Add some randomness to initial velocity
    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::new(0.0, 0.0, -1.0)).expr(), // Main direction
        speed: writer
            .rand(ScalarType::Float)
            .mul(writer.lit(0.3))
            .add(writer.lit(2.0))
            .expr(), // Random speed between 3-5
    };

    // Add acceleration to simulate particle physics
    let accel = LinearDragModifier::new(writer.lit(2.0).expr());

    // Create the spawner with a higher rate for a denser effect
    let spawner = Spawner::once(100.0.into(), false);

    // Combine everything into the effect
    let effect = effects.add(
        EffectAsset::new(1024, spawner, writer.finish())
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .update(accel)
            .render(ColorOverLifetimeModifier {
                gradient: color_gradient,
            })
            .render(SizeOverLifetimeModifier {
                gradient: size_gradient,
                screen_space_size: false,
            }),
    );

    *effect_assets = EffectAssets {
        thruster_gas: effect,
    };
}

pub struct ParticleEffectsPlugin;

impl Plugin for ParticleEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EffectAssets>()
            .add_systems(PostStartup, setup);
    }
}
