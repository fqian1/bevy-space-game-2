use std::sync::Once;

use bevy::prelude::*;
use bevy_hanabi::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct EffectAssets {
    pub thruster_gas: Handle<EffectAsset>,
}

fn setup(mut effects: ResMut<Assets<EffectAsset>>, mut effect_assets: ResMut<EffectAssets>) {
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::splat(1.0));
    color_gradient.add_key(0.1, Vec4::new(1.0, 1.0, 0.0, 1.0));
    color_gradient.add_key(0.4, Vec4::new(1.0, 0.0, 0.0, 1.0));
    color_gradient.add_key(1.0, Vec4::splat(0.0));

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, Vec3::splat(0.1));
    size_gradient.add_key(0.5, Vec3::splat(0.5));
    size_gradient.add_key(0.8, Vec3::splat(0.08));
    size_gradient.add_key(1.0, Vec3::splat(0.0));

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(0.06).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionCone3dModifier {
        base_radius: writer.lit(0.).expr(),
        top_radius: writer.lit(7.).expr(),
        height: writer.lit(20.).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(10.).expr(),
    };

    let spawner = Spawner::once(500.0.into(), false);

    let effect = effects.add(
        EffectAsset::new(32768, spawner, writer.finish())
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
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
