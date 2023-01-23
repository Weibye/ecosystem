use std::f32::consts::PI;

use agent::actions::{MoveAbility, MovementPath};
use bevy::prelude::*;
use bevy_atmosphere::{
    prelude::{AtmosphereModel, AtmospherePlugin, Nishita},
    system_param::AtmosphereMut,
};
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use bevy_turborand::{DelegatedRng, GlobalRng, RngPlugin};
use chronos::{Chrono, ChronoPlugin};
use fauna::{FaunaPlugin, SpawnFauna};
use flora::FloraPlugin;
use map::{
    plugin::MapPlugin,
    tiles::{world_to_index, MapIndex},
    Map, TileQuery,
};
use player::PlayerPlugin;
use resource::ResourcePlugin;
use utils::lerp;

mod agent;
mod chronos;
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
        .add_plugin(ChronoPlugin)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AtmosphereModel::default())
        .add_plugin(AtmospherePlugin)
        .add_startup_system_to_stage(AppStage::SpawnMap, setup)
        .add_system(draw_paths)
        .add_system(update_tile_pos)
        .add_system(daylight_cycle)
        .run();
}

#[derive(Component)]
struct Sun;

#[derive(Component)]
struct Moon;

#[derive(Component)]
struct SkyRoot;
// Elevation over time
// Azimuuth over time (0 -> 360)
// This will affect where shadows are
// luminance / strength
// how much sunlight / energy is transferred to the tiles / plants?

fn setup(
    mut cmd: Commands,
    mut rng: ResMut<GlobalRng>,
    // mut writer: EventWriter<SpawnFauna>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // map: Res<Map>,
) {
    const RADIUS: f32 = 9.0;
    // Sun starts straight below at 00:00
    const SUN_POSITION: Vec3 = Vec3::new(0.0, -RADIUS, 0.0);
    // Moon starts straight above at 00:00
    const MOON_POSITION: Vec3 = Vec3::new(0.0, RADIUS, 0.0); 

    // ambient light
    cmd.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.03,
    });

    cmd.spawn((
        PbrBundle {
            // SpatialBundle {
            transform: Transform::from_rotation(
                Quat::IDENTITY,
                // Quat::from_axis_angle(Vec3::Z, 45.0 * PI / 180.0),
            ),
            mesh: meshes.add(Mesh::from(shape::Plane::default())),
            ..default()
        },
        SkyRoot,
    ))
    .with_children(|root| {
        root.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere::default())),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgba(1.0, 0.0, 0.0, 0.5),
                    emissive: Color::YELLOW,
                    unlit: true,
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                }),
                transform: Transform::from_translation(SUN_POSITION), 

                ..default()
            },
            Sun,
        ));
        root.spawn((
            DirectionalLightBundle {
                directional_light: DirectionalLight {
                    illuminance: 10_000.0,
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_translation(SUN_POSITION).looking_at(Vec3::Y, Vec3::Z),
                ..default()
            },
            Sun,
        ));

        root.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.5,
                    ..default()
                })),
                material: materials.add(Color::BLUE.into()),
                transform: Transform::from_translation(MOON_POSITION), 
                ..default()
            },
            Moon,
        ));

        root.spawn((
            DirectionalLightBundle {
                directional_light: DirectionalLight {
                    illuminance: 5000.0,
                    shadows_enabled: true,
                    color: Color::rgb(0.3, 0.5, 0.3),
                    ..default()
                },
                transform: Transform::from_translation(MOON_POSITION).looking_at(-Vec3::Y, -Vec3::Z),
                ..default()
            },
            Moon,
        ));
    });

    // writer.send(SpawnFauna(Some(
    //     map.rand_from_query(
    //         rng.get_mut(),
    //         &TileQuery {
    //             walkable: Some(true),
    //             ..default()
    //         },
    //     )
    //     .unwrap(),
    // )));
}

fn daylight_cycle(
    mut atmosphere: AtmosphereMut<Nishita>,
    mut skyRoot: Query<&mut Transform, With<SkyRoot>>,
    mut sunlight: Query<&mut DirectionalLight, With<Sun>>,
    mut moonlight: Query<&mut DirectionalLight, With<Moon>>,
    mut ambient: ResMut<AmbientLight>,
    time: Res<Time>,
    chrono: Res<Chrono>,
) {
    // TODO: just spawn a plane that is slightly tilted, then rotate that along its local axis.

    let tilt_axis = Quat::from_axis_angle(Vec3::Z, 0.5 * PI) * Vec3::Y;

    let t = (chrono.day_progression as f32 * 2.0 * PI); // - (PI / 2.0);

    let sky_angle = lerp(chrono.day_progression, 0.0, 2.0 * std::f64::consts::PI) as f32; // + (2.0 / PI);

    //let day_ambient = ambient.brightness
    let mut light_modes = vec![
        // Day
        (
            (0.262 * (chrono.day_progression as f32 * 24.0) - 1.5).sin(),
            Vec4::new(1.0, 0.9, 1.0, 1.0),
        ),
        // Night
        (
            (0.262 * (chrono.day_progression as f32 * 24.0) + 1.5).sin(),
            Vec4::new(0.3, 0.3, 1.0, 1.0),
        ),
        // Twilight
        (
            (0.262 * 2.0 * (chrono.day_progression as f32 * 24.0) - 1.5).sin(),
            Vec4::new(1.0, 0.4, 0.0, 1.0),
        ),
    ];

    light_modes.sort_by(|current, next| next.0.partial_cmp(&current.0).unwrap());

    // ambient.brightness = light_modes.first().unwrap().0.max(0.0);

    // ambient.color = light_modes.get(0).unwrap().1.lerp(light_modes.get(1).unwrap().1, 1.0).into();

    //let ambient_night =  (0.262 * (chrono.day_progression as f32 * 24.0) + 1.5).sin();
    //let ambient_twilight = (0.262 * 2.0 * (chrono.day_progression as f32 * 24.0) - 1.5).sin();

    // if ambient_day > ambient_night && ambient_day > ambient_twilight {
    //     ambient.brightness = ambient_day.max(0.0);
    //     ambient.color = Color::rgb(1.0, 0.9, 1.0);
    //     info!("Day");
    // } else if ambient_night > ambient_day && ambient_night > ambient_twilight {
    //     ambient.brightness = ambient_night.max(0.0);
    //     ambient.color = Color::rgb(0.3, 0.3, 1.0);
    //     info!("Night");
    // } else {
    //     ambient.brightness = ambient_twilight.max(0.0);
    //     ambient.color = Color::rgb(1.0, 0.4, 0.0);
    //     info!("Twilight");
    // }

    // ambient.brightness = (0.262 * (chrono.day_progression as f32 * 24.0) - 1.5).sin().max(0.0);

    let mut transform = skyRoot.get_single_mut().unwrap();
    // Rotate around its
    transform.rotation = Quat::from_axis_angle(Vec3::Z, 45.0 * PI / 180.0)
        * Quat::from_axis_angle(Vec3::X, sky_angle);

    atmosphere.sun_position = transform.down();

    // if let Some((mut transform, mut light)) = q.single_mut().into() {
    //     transform.rotation = Quat::from_rotation_x(-t.sin().atan2(t.cos()));
    //     light.illuminance = t.sin().max(0.0).powf(2.0) * 100_000.0;
    // }
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
