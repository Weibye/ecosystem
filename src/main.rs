use bevy::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng, RngPlugin};
use fauna::{FaunaPlugin, SpawnFauna};
use flora::FloraPlugin;
use landscape::{get_rand_pos, LandscapePlugin, TileSettings};
use player::PlayerPlugin;
use resource::ResourcePlugin;

mod agent;
mod fauna;
mod flora;
mod landscape;
mod player;
mod resource;
mod utils;

#[derive(StageLabel)]
enum AppStage {
    SeedBoard,
    SpawnFlora,
    SpawnFauna,
}

fn main() {
    App::new()
        .add_startup_stage(AppStage::SeedBoard, SystemStage::parallel())
        .add_startup_stage_after(
            AppStage::SeedBoard,
            AppStage::SpawnFlora,
            SystemStage::parallel(),
        )
        .add_startup_stage_after(
            AppStage::SpawnFlora,
            AppStage::SpawnFauna,
            SystemStage::parallel(),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(RngPlugin::default())
        .add_plugin(LandscapePlugin::default())
        .add_plugin(FaunaPlugin)
        .add_plugin(FloraPlugin)
        .add_plugin(ResourcePlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut cmd: Commands,
    mut rng: ResMut<GlobalRng>,
    mut writer: EventWriter<SpawnFauna>,
    settings: Res<TileSettings>,
) {
    // ambient light
    cmd.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.04,
    });

    // Spawn light
    cmd.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    writer.send(SpawnFauna {
        position: Some(get_rand_pos(rng.get_mut(), &settings)),
    });
}
