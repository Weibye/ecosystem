use bevy::prelude::*;
use fauna::{FaunaPlugin, SpawnFauna};
use random::{Random, RandomPlugin};
use resource::ResourcePlugin;
use utils::get_rand_point_on_board;

mod agent;
mod fauna;
mod random;
mod resource;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FaunaPlugin)
        .add_plugin(ResourcePlugin)
        .add_plugin(RandomPlugin)
        .insert_resource(Board(Vec2::new(10.0, 10.0)))
        .add_startup_system(setup)
        .run();
}

struct Board(pub Vec2);

fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<Random>,
    board: Res<Board>,
    mut writer: EventWriter<SpawnFauna>,
) {
    // Spawn camera
    cmd.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn ground
    cmd.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(Color::rgb(0.2, 1.0, 0.3).into()),
        ..default()
    });

    // ambient light
    cmd.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.04,
    });

    // Spawn light
    cmd.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    writer.send(SpawnFauna {
        position: Some(get_rand_point_on_board(&mut rng.0, &board)),
    });
}
