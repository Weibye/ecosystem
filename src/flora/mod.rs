use bevy::{
    prelude::{
        default, info, shape, App, Assets, Color, Commands, Component, EventReader, EventWriter,
        Mesh, PbrBundle, Plugin, Query, Res, ResMut, StandardMaterial,
        Transform, Vec3, IntoSystemConfigs,
    },
    utils::HashMap,
};
use bevy_mod_picking::PickableBundle;
use bevy_turborand::{DelegatedRng, GlobalRng, TurboRand};

use crate::{
    map::{
        tiles::{MapIndex, TileType},
        Map, TileQuery,
    },
    resource::{FoodSource, WaterSource},
    utils::lerp_range,
    WorldCreationSet,
};

pub(crate) struct FloraPlugin;

// Plants grow and become food. The more they grow, the more food they contain.
impl Plugin for FloraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnFlora>()
            .add_systems((generate_flora, spawn_water).in_set(WorldCreationSet::SpawnFlora))
            .add_systems((grow_flora, spread_flora, spawn_flora).chain())
            .add_system(scale_flora.after(grow_flora));
    }
}

const FOOD_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);
const WATER_COLOR: Color = Color::rgb(0.0, 0.0, 1.0);

// Event that spawns a new flora at a map location.
struct SpawnFlora(usize);

// Plants should be able to grow
// More grown plants should provide more foot
// In the future, add more conditions

// Flora Archetypes:
// Grass / weed: Growth = food
//  When eaten, reduces food and growth, and will either be destroyed or keep growing
// Bushes / trees: Spends time growing until fully grown, then will start producing food on a cycle.
//  When eaten, reduces food but not growth.

#[derive(Component)]
struct Flora {
    /// The speed at which the flora grows each cycle.
    growing_speed: f32,
    /// The current growth of the flora. Range: 0.0..=1.0
    current_growth: f32,
}

/// Based on local growing conditions, flora should grow this cycle.
fn grow_flora(mut q: Query<&mut Flora>) {
    for mut flora in &mut q {
        if flora.current_growth == 1.0 {
            continue;
        }
        flora.current_growth = (flora.current_growth + flora.growing_speed).clamp(0.0, 1.0);
    }
}

fn scale_flora(mut q: Query<(&mut Transform, &Flora)>) {
    for (mut transform, flora) in &mut q {
        transform.scale = get_flora_scale(flora.current_growth);
    }
}

fn get_flora_scale(growth: f32) -> Vec3 {
    Vec3::ONE * lerp_range(growth, &(0.1..2.0))
}

const THRESHOLD: f32 = 2.0;

/// Based on the current growth of the flora, it should spread its seeds to nearby tiles.
fn spread_flora(q: Query<(&Flora, &MapIndex)>, map: Res<Map>, mut event: EventWriter<SpawnFlora>) {
    let existing_flora: HashMap<usize, &Flora> = q
        .iter()
        .map(|(flora, map_index)| (map_index.0, flora))
        .collect();

    // Find all growable tiles that does not yet have flora on them, but have flora on neighboring tiles.
    let growable_tiles: Vec<usize> = map
        .query(&TileQuery {
            growable: Some(true),
            ..default()
        })
        .iter()
        .map(|(index, _)| *index)
        .collect();

    let mut spread_score: Vec<(usize, f32)> = vec![];
    for tile in growable_tiles {
        // If this tile already have a flora on it, set score to 0
        if existing_flora.iter().any(|(index, _)| *index == tile) {
            spread_score.push((tile, 0.0));
        } else {
            let mut sum = 0.0;
            let neighbours = map.query_neighbours(
                tile,
                &TileQuery {
                    growable: Some(true),
                    ..default()
                },
            );
            for neighbour in neighbours {
                // Sum the total growth score of all neighbouring tiles
                // Which informs the chance of spawning a new flora on that tile
                if existing_flora.contains_key(&neighbour) {
                    sum += &existing_flora[&neighbour].current_growth;
                }
            }
            spread_score.push((tile, sum));
        }
    }

    for (index, score) in spread_score {
        if score > THRESHOLD {
            event.send(SpawnFlora(index));
        }
    }
}

fn spawn_flora(
    mut cmd: Commands,
    mut event: EventReader<SpawnFlora>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut glob_rng: ResMut<GlobalRng>,
    map: Res<Map>,
) {
    let rng = glob_rng.get_mut();

    for event in event.iter() {
        info!("Spawning flora for tile {:?}", event.0);
        let flora = Flora {
            growing_speed: rng.f32() * 0.01,
            current_growth: rng.f32() * 0.5,
        };

        cmd.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                material: materials.add(FOOD_COLOR.into()),
                transform: Transform {
                    translation: map.index_to_world(event.0.into()),
                    scale: get_flora_scale(flora.current_growth),
                    ..default()
                },
                ..default()
            },
            FoodSource {
                content: flora.current_growth * 100.0,
            },
            flora,
            MapIndex(event.0),
            PickableBundle::default(),
        ));
    }
}

/// Spawns food on all grass tiles
fn generate_flora(
    map: Res<Map>,
    mut glob_rng: ResMut<GlobalRng>,
    mut event: EventWriter<SpawnFlora>,
) {
    info!("spawning flora");
    let rng = glob_rng.get_mut();
    let tiles = map.query(&TileQuery {
        growable: Some(true),
        ..default()
    });

    for (index, _) in tiles {
        if rng.f32() * 100.0 > 80.0 {
            info!("Generating flora for tile {:?}", index);
            event.send(SpawnFlora(index));
        }
    }
}

// TODO: Move this to resources instead of flora
fn spawn_water(
    map: Res<Map>,
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("spawning water");
    for n in 0..map.tiles.len() {
        if map.tiles[n] == TileType::Water {
            cmd.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                    material: materials.add(WATER_COLOR.into()),
                    transform: Transform::from_translation(map.index_to_world(n.into())),
                    ..default()
                },
                WaterSource { content: 100.0 },
                MapIndex(n),
                PickableBundle::default(),
            ));
        }
    }
}
