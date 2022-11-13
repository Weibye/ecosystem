//! Collection of various utility functions.

use std::ops::Range;

use bevy::prelude::{Vec2, Vec3};
use bevy_turborand::{rng::Rng, TurboRand};

use crate::Board;

/// From a set of points, return whichever point is closest to the target.
#[allow(dead_code)]
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
pub(crate) fn get_rand_point_on_board(rng: &mut Rng, board: &Board) -> Vec2 {
    let half_x = board.0.x / 2.0;
    let half_y = board.0.y / 2.0;
    Vec2::new(
        lerp_range(rng.f32(), -half_x..half_x),
        lerp_range(rng.f32(), -half_y..half_y),
    )
}

pub(crate) fn lerp(value: f32, min: f32, max: f32) -> f32 {
    min + value * (max - min)
}

pub(crate) fn lerp_range(value: f32, range: Range<f32>) -> f32 {
    range.start + value * (range.end - range.start)
}
