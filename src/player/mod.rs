use bevy::prelude::{
    default, info, App, Camera3dBundle, Commands, Component, KeyCode, Plugin, Query, Transform,
    Vec3, With,
};
use bevy_mod_picking::{DefaultPickingPlugins, PickingCameraBundle, Selection};
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin, InputMap, VirtualDPad},
    Actionlike, InputManagerBundle,
};

use crate::{
    fauna::needs::{Health, Hunger, Reproduction, Thirst},
    resource::{FoodSource, WaterSource},
    utils::project_to_plane,
};

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins)
            // This plugin maps inputs to an input-type agnostic action-state
            // We need to provide it with an enum which stores the possible actions a player could take
            .add_plugin(InputManagerPlugin::<Action>::default())
            .add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(output_fauna_data)
            .add_system(output_flora_data);
    }
}

#[derive(Component, Debug)]
struct Player;

#[derive(Actionlike, Clone, Debug)]
enum Action {
    Move,
}

fn spawn_player(mut cmd: Commands) {
    // Spawn camera
    cmd.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PickingCameraBundle::default(),
        InputManagerBundle::<Action> {
            input_map: InputMap::new([(
                VirtualDPad {
                    up: KeyCode::W.into(),
                    down: KeyCode::S.into(),
                    left: KeyCode::A.into(),
                    right: KeyCode::D.into(),
                },
                Action::Move,
            )])
            .build(),
            ..default()
        },
        Player,
    ));
}

const SPEED: f32 = 0.1;

fn move_player(mut q: Query<(&ActionState<Action>, &mut Transform), With<Player>>) {
    for (action, mut transform) in &mut q {
        if action.pressed(Action::Move) {
            let axis_pair = action.axis_pair(Action::Move).unwrap();
            let forward = project_to_plane(transform.forward(), Vec3::Y).normalize();
            let right = project_to_plane(transform.right(), Vec3::Y).normalize();

            transform.translation +=
                forward * axis_pair.y() * SPEED + right * axis_pair.x() * SPEED;
        }
    }
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
