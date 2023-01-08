use bevy::{
    prelude::{
        default, info, shape, App, Assets, Camera3dBundle, Color, Commands, KeyCode, Mesh,
        PbrBundle, Plugin, Query, ResMut, StandardMaterial,
    },
};
use bevy_mod_picking::{DefaultPickingPlugins, PickingCameraBundle, Selection};
use leafwing_input_manager::{
    axislike::VirtualAxis,
    prelude::{InputMap, SingleAxis, VirtualDPad},
    InputManagerBundle,
};

use crate::{
    fauna::needs::{Health, Hunger, Reproduction, Thirst},
    resource::{FoodSource, WaterSource},
};

use self::camera_controller::{
    CameraController, CameraControllerPlugin, CameraControllerSettings, CameraMovement,
    CameraTarget,
};

mod camera_controller;

/// The PlayerPlugin governs everything that has to do with how the player interacts with the simulation.
pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins)
            .add_plugin(CameraControllerPlugin {
                settings: CameraControllerSettings {
                    translation_speed: 0.1,
                    rotation_speed: 0.05,
                    zoom_speed: 0.04,
                    zoom: 5.0..30.0,
                },
            })
            .add_startup_system(spawn_player)
            .add_system(output_fauna_data)
            .add_system(output_flora_data);
    }
}

fn spawn_player(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn camera target

    let target = cmd
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                material: materials.add(Color::PINK.into()),
                ..default()
            },
            CameraTarget,
        ))
        .id();

    //  Spawn camera
    cmd.spawn((
        Camera3dBundle::default(),
        PickingCameraBundle::default(),
        InputManagerBundle::<CameraMovement> {
            input_map: InputMap::default()
                .insert(
                    VirtualDPad {
                        up: KeyCode::W.into(),
                        down: KeyCode::S.into(),
                        left: KeyCode::A.into(),
                        right: KeyCode::D.into(),
                    },
                    CameraMovement::Move,
                )
                .insert(
                    VirtualAxis {
                        negative: KeyCode::Q.into(),
                        positive: KeyCode::E.into(),
                    },
                    CameraMovement::Rotate,
                )
                .insert(SingleAxis::mouse_wheel_y(), CameraMovement::Zoom)
                // Add action to increase and decrease simulation speed
                .build(),
            ..default()
        },
        CameraController::new(target),
    ));
}

fn output_fauna_data(q: Query<(&Selection, &Hunger, &Thirst, &Reproduction, &Health)>) {
    for (selection, hunger, thirst, reproduction, health) in &q {
        if !selection.selected() {
            continue;
        }

        info!(
            "\nHunger: {:?}\nThirst: {:?}\nReproduction: {:?}\nHealth: {:?}",
            hunger.value, thirst.value, reproduction.value, health.value
        );
    }
}

fn output_flora_data(q: Query<(&Selection, Option<&FoodSource>, Option<&WaterSource>)>) {
    for (selection, food_source, water_source) in &q {
        if !selection.selected() {
            continue;
        }
        if food_source.is_none() && water_source.is_none() {
            continue;
        }

        if let Some(food) = food_source {
            info!("Food: {:?}", food.content);
        }

        if let Some(water) = water_source {
            info!("Water: {:?}", water.content);
        }
    }
}
