//! Collection of various utility functions.

use bevy::prelude::Vec3;
use std::ops::Range;

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

/// Linearly interpolates between two values.
/// Value should be in range 0..=1.
pub(crate) fn lerp(value: f32, min: f32, max: f32) -> f32 {
    min + value * (max - min)
}

/// Linearly interpolates between two values.
/// Value should be in range 0..=1.
pub(crate) fn lerp_range(value: f32, range: Range<f32>) -> f32 {
    lerp(value, range.start, range.end)
}
