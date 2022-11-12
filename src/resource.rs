use bevy::prelude::{
    default, shape, Assets, Changed, Color, Commands, Component, Entity, Mesh, PbrBundle, Plugin,
    Query, Res, ResMut, StandardMaterial, Transform,
};

use crate::{random::Random, utils::get_rand_point_on_board, Board};

// RESOURCES
pub(crate) struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_resource)
            .add_system(remove_empty_food)
            .add_system(remove_empty_water);
    }
}

#[derive(Component, Debug, Copy, Clone)]
pub(crate) struct FoodSource {
    /// How much food this contains
    pub content: f32,
}

#[derive(Component, Debug, Copy, Clone)]
pub(crate) struct WaterSource {
    /// How much water this contains
    pub content: f32,
}

fn spawn_resource(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    board: Res<Board>,
    mut rng: ResMut<Random>,
) {
    // FOOD
    for _ in 0..10 {
        let point = get_rand_point_on_board(&mut rng.0, &board);
        cmd.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(point.x, 1.3, point.y),
            ..default()
        })
        .insert(FoodSource { content: 5.0 });
    }

    for _ in 0..10 {
        let point = get_rand_point_on_board(&mut rng.0, &board);
        cmd.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
            transform: Transform::from_xyz(point.x, 1.3, point.y),
            ..default()
        })
        .insert(WaterSource { content: 5.0 });
    }
}

/// Removes any food that have become empty.
fn remove_empty_food(mut cmd: Commands, q: Query<(Entity, &FoodSource), Changed<FoodSource>>) {
    for (entity, food) in &q {
        // info("")
        if food.content <= 0.0 {
            cmd.entity(entity).despawn();
        }
    }
}

/// Removes any water that have become empty.
fn remove_empty_water(mut cmd: Commands, q: Query<(Entity, &WaterSource), Changed<WaterSource>>) {
    for (entity, water) in &q {
        // info("")
        if water.content <= 0.0 {
            cmd.entity(entity).despawn();
        }
    }
}
