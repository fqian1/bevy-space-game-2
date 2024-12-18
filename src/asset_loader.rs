use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct ImageAssets {
    // Asteroid assets
    pub asteroid_base: Handle<Image>,
    pub asteroid_explode: Handle<Image>,

    // Background assets
    pub background_void: Handle<Image>,
    pub background_stars_1: Handle<Image>,
    pub background_stars_2: Handle<Image>,
    pub background_solid_color: Handle<Image>,
    pub background_shadows_1: Handle<Image>,
    pub background_shadows_2: Handle<Image>,
    pub background_stars_3: Handle<Image>,
    pub background_stars_4: Handle<Image>,
    pub background_big_star_1: Handle<Image>,
    pub background_big_star_2: Handle<Image>,
    pub background_black_hole: Handle<Image>,
    pub background_rotary_star_1: Handle<Image>,
    pub background_rotary_star_2: Handle<Image>,

    // Effect assets
    pub asteroid_flame: Handle<Image>,

    // Main ship base assets
    pub ship_base_full_health: Handle<Image>,
    pub ship_base_slight_damage: Handle<Image>,
    pub ship_base_damaged: Handle<Image>,
    pub ship_base_very_damaged: Handle<Image>,

    // Main ship engine assets
    pub ship_engine_base_idle: Handle<Image>,
    pub ship_engine_base_powering: Handle<Image>,
    pub ship_engine_base_spritesheet: Handle<Image>,
    pub ship_engine_big_pulse_idle: Handle<Image>,
    pub ship_engine_big_pulse_powering: Handle<Image>,
    pub ship_engine_big_pulse_spritesheet: Handle<Image>,
    pub ship_engine_burst_idle: Handle<Image>,
    pub ship_engine_burst_powering: Handle<Image>,
    pub ship_engine_burst_spritesheet: Handle<Image>,
    pub ship_engine_supercharged_idle: Handle<Image>,
    pub ship_engine_supercharged_powering: Handle<Image>,
    pub ship_engine_supercharged_spritesheet: Handle<Image>,

    // Main ship engine base assets
    pub ship_engine_base: Handle<Image>,
    pub ship_engine_big_pulse: Handle<Image>,
    pub ship_engine_burst: Handle<Image>,
    pub ship_engine_supercharged: Handle<Image>,

    // Main ship shield assets
    pub ship_shield_front: Handle<Image>,
    pub ship_shield_front_and_side: Handle<Image>,
    pub ship_shield_invincibility: Handle<Image>,
    pub ship_shield_round: Handle<Image>,

    // Main ship weapon assets
    pub ship_weapon_auto_cannon: Handle<Image>,
    pub ship_weapon_big_space_gun: Handle<Image>,
    pub ship_weapon_rockets: Handle<Image>,
    pub ship_weapon_zapper: Handle<Image>,

    // Main ship weapon projectile assets
    pub projectile_auto_cannon: Handle<Image>,
    pub projectile_big_space_gun: Handle<Image>,
    pub projectile_rocket: Handle<Image>,
    pub projectile_zapper: Handle<Image>,

    // Planet assets
    pub planet_earth_like: Handle<Image>,
    pub planet_earth_like_no_glow: Handle<Image>,
}

fn load_assets(mut scene_assets: ResMut<ImageAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = ImageAssets {
        // Asteroid assets
        asteroid_base: asset_server.load("Asteroids/PNGs/Asteroid 01 - Base.png"),
        asteroid_explode: asset_server.load("Asteroids/PNGs/Asteroid 01 - Explode.png"),

        // Background assets
        background_void: asset_server.load("Backgrounds/PNGs/Condesed/Starry background  - Layer 01 - Void.png"),
        background_stars_1: asset_server.load("Backgrounds/PNGs/Condesed/Starry background  - Layer 02 - Stars.png"),
        background_stars_2: asset_server.load("Backgrounds/PNGs/Condesed/Starry background  - Layer 03 - Stars.png"),
        background_solid_color: asset_server.load("Backgrounds/PNGs/Split up/Starry background  - Layer 01 - Solid colour.png"),
        background_shadows_1: asset_server.load("Backgrounds/PNGs/Split up/Starry background  - Layer 02 - Shadows.png"),
        background_shadows_2: asset_server.load("Backgrounds/PNGs/Split up/Starry background  - Layer 02 - Shadows 2.png"),
        background_stars_3: asset_server.load("Backgrounds/PNGs/Split up/Starry background  - Layer 03 - Stars.png"),
        background_stars_4: asset_server.load("Backgrounds/PNGs/Split up/Starry background  - Layer 03 - Stars 2.png"),
        background_big_star_1: asset_server.load("Backgrounds/PNGs/Split up/Starry background - Layer X - Big Star.png"),
        background_big_star_2: asset_server.load("Backgrounds/PNGs/Split up/Starry background - Layer X - Big Star 2.png"),
        background_black_hole: asset_server.load("Backgrounds/PNGs/Split up/Starry background - Layer X -Black hole.png"),
        background_rotary_star_1: asset_server.load("Backgrounds/PNGs/Split up/Starry background - Layer X -Rotary Star.png"),
        background_rotary_star_2: asset_server.load("Backgrounds/PNGs/Split up/Starry background - Layer X -Rotary Star 2.png"),

        // Effect assets
        asteroid_flame: asset_server.load("Effects/PNGs/Asteroid - Flame.png"),

        // Main ship base assets
        ship_base_full_health: asset_server.load("Main Ship/Main Ship - Bases/PNGs/Main Ship - Base - Full health.png"),
        ship_base_slight_damage: asset_server.load("Main Ship/Main Ship - Bases/PNGs/Main Ship - Base - Slight damage.png"),
        ship_base_damaged: asset_server.load("Main Ship/Main Ship - Bases/PNGs/Main Ship - Base - Damaged.png"),
        ship_base_very_damaged: asset_server.load("Main Ship/Main Ship - Bases/PNGs/Main Ship - Base - Very damaged.png"),

        // Main ship engine assets
        ship_engine_base_idle: asset_server.load("Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Base Engine - Idle.png"),
        ship_engine_base_powering: asset_server.load("Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Base Engine - Powering.png"),
        ship_engine_base_spritesheet: asset_server.load("Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Base Engine - Spritesheet.png"),
        ship_engine_big_pulse_idle: asset_server.load("Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Big Pulse Engine - Idle.png"),
        ship_engine_big_pulse_powering: asset_server.load("Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Big Pulse Engine - Powering.png"),
        ship_engine_big_pulse_spritesheet: asset_server.load("Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Big Pulse Engine - Spritesheet.png"),
        ship_engine_burst_idle: asset_server.load("Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Burst Engine - Idle.png"),
        ship_engine_burst_powering: asset_server.load("Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Burst Engine - Powering.png"),
        ship_engine_burst_spritesheet: asset_server.load("Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Burst Engine - Spritesheet.png"),
        ship_engine_supercharged_idle: asset_server.load("Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Supercharged Engine - Idle.png"),
        ship_engine_supercharged_powering: asset_server.load("Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Supercharged Engine - Powering.png"),
        ship_engine_supercharged_spritesheet: asset_server.load("Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Supercharged Engine - Spritesheet.png"),

        // Main ship engine base assets
        ship_engine_base: asset_server.load("Main Ship/Main Ship - Engines/PNGs/Main Ship - Engines - Base Engine.png"),
        ship_engine_big_pulse: asset_server.load("Main Ship/Main Ship - Engines/PNGs/Main Ship - Engines - Big Pulse Engine.png"),
        ship_engine_burst: asset_server.load("Main Ship/Main Ship - Engines/PNGs/Main Ship - Engines - Burst Engine.png"),
        ship_engine_supercharged: asset_server.load("Main Ship/Main Ship - Engines/PNGs/Main Ship - Engines - Supercharged Engine.png"),

        // Main ship shield assets
        ship_shield_front: asset_server.load("Main Ship/Main Ship - Shields/PNGs/Main Ship - Shields - Front Shield.png"),
        ship_shield_front_and_side: asset_server.load("Main Ship/Main Ship - Shields/PNGs/Main Ship - Shields - Front and Side Shield.png"),
        ship_shield_invincibility: asset_server.load("Main Ship/Main Ship - Shields/PNGs/Main Ship - Shields - Invincibility Shield.png"),
        ship_shield_round: asset_server.load("Main Ship/Main Ship - Shields/PNGs/Main Ship - Shields - Round Shield.png"),

        // Main ship weapon assets
        ship_weapon_auto_cannon: asset_server.load("Main Ship/Main Ship - Weapons/PNGs/Main Ship - Weapons - Auto Cannon.png"),
        ship_weapon_big_space_gun: asset_server.load("Main Ship/Main Ship - Weapons/PNGs/Main Ship - Weapons - Big Space Gun.png"),
        ship_weapon_rockets: asset_server.load("Main Ship/Main Ship - Weapons/PNGs/Main Ship - Weapons - Rockets.png"),
        ship_weapon_zapper: asset_server.load("Main Ship/Main Ship - Weapons/PNGs/Main Ship - Weapons - Zapper.png"),

        // Main ship weapon projectile assets
        projectile_auto_cannon: asset_server.load("Main ship weapons/PNGs/Main ship weapon - Projectile - Auto cannon bullet.png"),
        projectile_big_space_gun: asset_server.load("Main ship weapons/PNGs/Main ship weapon - Projectile - Big Space Gun.png"),
        projectile_rocket: asset_server.load("Main ship weapons/PNGs/Main ship weapon - Projectile - Rocket.png"),
        projectile_zapper: asset_server.load("Main ship weapons/PNGs/Main ship weapon - Projectile - Zapper.png"),

        // Planet assets
        planet_earth_like: asset_server.load("Planets/PNGs/Earth-Like planet.png"),
        planet_earth_like_no_glow: asset_server.load("Planets/PNGs/Earth-Like planet - Without back glow.png"),
    }
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ImageAssets>()
            .add_systems(Startup, load_assets);
    }
}
