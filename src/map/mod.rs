use bevy::prelude::{Resource, Vec3};
use bracket_pathfinding::prelude::Algorithm2D;

use self::{
    plugin::MapSettings,
    tiles::{pos_to_world, TileType},
};

pub(crate) mod pathfinding;
pub(crate) mod plugin;
pub(crate) mod tiles;

#[derive(Resource)]
pub(crate) struct Map {
    pub(crate) tiles: Vec<TileType>,
    pub(crate) settings: MapSettings,
}

impl Map {
    pub(crate) fn index_to_world(&self, index: usize) -> Vec3 {
        pos_to_world(self.index_to_point2d(index).into(), &self.settings)
    }
}
