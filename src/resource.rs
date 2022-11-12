use bevy::prelude::{
    default, shape, Assets, Changed, Color, Commands, Component, Entity, Mesh, NonSendMut,
    PbrBundle, Plugin, Query, Res, ResMut, StandardMaterial, Transform,
};

use crate::{random::Random, utils::get_rand_point_on_board, Board};

// RESOURCES
pub(crate) struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_resource)
            .add_system(remove_empty_food);
    }
}

#[derive(Component)]
enum Resource {
    Food,
    Water,
}

#[derive(Component, Debug, Copy, Clone)]
pub(crate) struct FoodSource {
    /// How much food this contains
    content: f32,
}

fn spawn_resource(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    board: Res<Board>,
    mut rng: ResMut<Random>,
) {
    for _ in 0..10 {
        let point = get_rand_point_on_board(&mut rng.0, &board);
        cmd.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: 0.2,
                ..default()
            })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(point.x, 0.3, point.y),
            ..default()
        })
        .insert(FoodSource { content: 40.0 });
    }
}

/// Removes any food that have become empty.
fn remove_empty_food(mut cmd: Commands, q: Query<(Entity, &FoodSource), Changed<FoodSource>>) {
    for (entity, food) in &q {
        if food.content <= 0.0 {
            cmd.entity(entity).despawn();
        }
    }
}
