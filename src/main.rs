use agent::actions::{MoveAbility, MovementPath};
use bevy::prelude::*;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use bevy_turborand::{DelegatedRng, GlobalRng, RngPlugin};
use chronos::ChronoPlugin;
use fauna::{FaunaPlugin, SpawnFauna};
use flora::FloraPlugin;
use map::{
    plugin::MapPlugin,
    tiles::{world_to_index, MapIndex},
    Map, TileQuery,
};
use player::PlayerPlugin;
use resource::ResourcePlugin;

mod agent;
mod chronos;
mod fauna;
mod flora;
mod map;
mod player;
mod resource;
mod utils;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum WorldCreationSet {
    SeedMap,
    SpawnMap,
    SpawnFlora,
    SpawnFauna,
}



fn main() {
    App::new()
        .configure_set(
            (
                WorldCreationSet::SeedMap,
                WorldCreationSet::SpawnMap,
                WorldCreationSet::SpawnFlora,
                WorldCreationSet::SpawnFauna
            ).chain()
            .in_base_set(StartupSet::Startup))
        .add_plugins(DefaultPlugins)
        .add_plugin(RngPlugin::default())
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(MapPlugin {
            tile_size: 1.0,
            map_size: (16, 16),
        })
        .add_plugin(FaunaPlugin)
        .add_plugin(FloraPlugin)
        .add_plugin(ResourcePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ChronoPlugin)
        .add_startup_system_to_stage(WorldCreationSet::SpawnMap, setup)
        .add_system(draw_paths)
        .add_system(update_tile_pos)
        .run();
}

fn setup(
    mut cmd: Commands,
    mut rng: ResMut<GlobalRng>,
    mut writer: EventWriter<SpawnFauna>,
    map: Res<Map>,
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

    writer.send(SpawnFauna(Some(
        map.rand_from_query(
            rng.get_mut(),
            &TileQuery {
                walkable: Some(true),
                ..default()
            },
        )
        .unwrap(),
    )));
}

fn draw_paths(
    q: Query<(&GlobalTransform, &MovementPath)>,
    mut lines: ResMut<DebugLines>,
    map: Res<Map>,
) {
    for (transform, path) in &q {
        for n in 0..path.path.len() {
            if n == 0 {
                lines.line(
                    transform.translation(),
                    map.index_to_world(path.path[0].into()),
                    0.0,
                );
            } else {
                lines.line(
                    map.index_to_world(path.path[n - 1].into()),
                    map.index_to_world(path.path[n].into()),
                    0.0,
                );
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_tile_pos(
    mut q: Query<(&mut MapIndex, &GlobalTransform), (With<MoveAbility>, Changed<GlobalTransform>)>,
    map: Res<Map>,
) {
    for (mut index, transform) in &mut q {
        *index = world_to_index(&transform.translation(), &map);
    }
}
