use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct ImageAssets {
    pub spaceship: Handle<Image>,
    pub missile: Handle<Image>,
    pub asteroid: Handle<Image>,
}

fn load_assets(mut scene_assets: ResMut<ImageAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = ImageAssets {
        spaceship: asset_server.load("Main Ship/Main Ship - Bases/PNGs/Main Ship - Base - Full health.png"),
        missile: asset_server .load("Main ship weapons/PNGs/Main ship weapon - Projectile - Rocket.png"),
        asteroid: asset_server.load("Asteroids/PNGs/Asteroid 01 - Base.png"),
    }
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ImageAssets>()
            .add_systems(Startup, load_assets);
    }
}
