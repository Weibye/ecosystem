use bevy::prelude::{Component, Plugin};

pub(crate) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        todo!()
    }
}

#[derive(Component)]
pub(crate) struct Movement {}

// When hungry
// Find a random tile with food
// Generate a A* path to the tile
// Move through the path until reaching the finish
// Debug lines showing the path
