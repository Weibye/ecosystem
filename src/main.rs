use agent::actions::{MoveAbility, MovementPath};
use bevy::prelude::*;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use bevy_turborand::{DelegatedRng, GlobalRng, RngPlugin};
use fauna::{FaunaPlugin, SpawnFauna};
use flora::FloraPlugin;
use map::{
    plugin::MapPlugin,
    tiles::{create_rand_pos, world_to_pos, TilePos},
    Map,
};
use player::PlayerPlugin;
use resource::ResourcePlugin;

mod agent;
mod fauna;
mod flora;
mod map;
mod player;
mod resource;
mod utils;

#[derive(StageLabel)]
enum AppStage {
    SeedMap,
    SpawnMap,
    SpawnFlora,
    SpawnFauna,
}

fn main() {
    App::new()
        .add_startup_stage(AppStage::SeedMap, SystemStage::parallel())
        .add_startup_stage_after(
            AppStage::SeedMap,
            AppStage::SpawnMap,
            SystemStage::parallel(),
        )
        .add_startup_stage_after(
            AppStage::SeedMap,
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
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(MapPlugin {
            tile_size: 1.0,
            map_size: (16, 16),
        })
        .add_plugin(FaunaPlugin)
        .add_plugin(FloraPlugin)
        .add_plugin(ResourcePlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system_to_stage(AppStage::SpawnMap, setup)
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

    writer.send(SpawnFauna {
        position: Some(create_rand_pos(rng.get_mut(), &map.settings)),
    });
}

fn draw_paths(q: Query<&MovementPath>, mut lines: ResMut<DebugLines>, map: Res<Map>) {
    for path in &q {
        for n in 0..path.path.len() {
            if n == 0 {
                continue;
            }
            lines.line(
                map.index_to_world(path.path[n - 1]),
                map.index_to_world(path.path[n]),
                0.0,
            );
        }
    }
}

fn update_tile_pos(
    #[allow(clippy::type_complexity)]
    mut q: Query<
        (&mut TilePos, &GlobalTransform),
        (With<MoveAbility>, Changed<GlobalTransform>),
    >,
    map: Res<Map>,
) {
    for (mut tile_pos, transform) in &mut q {
        tile_pos.pos = world_to_pos(&transform.translation(), &map.settings);
    }
}
