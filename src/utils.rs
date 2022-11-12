//! Collection of various utility functions.

use bevy::prelude::{Vec2, Vec3};
use rand::{
    rngs::{SmallRng, StdRng},
    Rng,
};

use crate::Board;

/// From a set of points, return whichever point is closest to the target.
pub(crate) fn closest(points: &mut dyn Iterator<Item = Vec3>, target: Vec3) -> Vec3 {
    points
        .min_by(|a, b| {
            let a_distance = (*a - target).length_squared();
            let b_distance = (*b - target).length_squared();
            a_distance.partial_cmp(&b_distance).unwrap()
        })
        .expect("Should have a closest point")
}

/// Gets a random point on the board
pub(crate) fn get_rand_point_on_board(rng: &mut SmallRng, board: &Board) -> Vec2 {
    let half_x = board.0.x / 2.0;
    let half_y = board.0.y / 2.0;
    Vec2::new(
        rng.gen_range(-half_x..=half_x),
        rng.gen_range(-half_y..=half_y),
    )
}
