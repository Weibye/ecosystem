use bevy::prelude::{Changed, Commands, Component, Entity, Plugin, Query};

// RESOURCES
pub(crate) struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(remove_empty_food)
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
