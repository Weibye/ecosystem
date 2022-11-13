use bevy::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng, RngPlugin};
use fauna::{FaunaPlugin, SpawnFauna};
use landscape::LandscapePlugin;
use resource::ResourcePlugin;
use utils::get_rand_point_on_board;

mod agent;
mod fauna;
mod landscape;
mod resource;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RngPlugin::default())
        .add_plugin(LandscapePlugin::default())
        .add_plugin(FaunaPlugin)
        .add_plugin(ResourcePlugin)
        .insert_resource(Board(Vec2::new(10.0, 10.0)))
        .add_startup_system(setup)
        .run();
}

#[derive(Resource)]
struct Board(pub Vec2);

fn setup(
    mut cmd: Commands,
    mut rng: ResMut<GlobalRng>,
    board: Res<Board>,
    mut writer: EventWriter<SpawnFauna>,
) {
    // Spawn camera
    cmd.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

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
        position: Some(get_rand_point_on_board(&mut *rng.get_mut(), &board)),
    });
}
