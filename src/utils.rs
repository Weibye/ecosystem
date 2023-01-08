//! Collection of various utility functions.

use bevy::prelude::Vec3;
use bracket_pathfinding::prelude::Point;
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
///
/// Value should be in range 0..=1.
pub(crate) fn lerp(value: f32, min: f32, max: f32) -> f32 {
    min + value * (max - min)
}

/// Linearly interpolates between two values.
///
/// Value should be in range 0..=1.
pub(crate) fn lerp_range(value: f32, range: &Range<f32>) -> f32 {
    lerp(value, range.start, range.end)
}

/// Project a vector onto to a plane with the given normal.
pub(crate) fn project_to_plane(vector: Vec3, normal: Vec3) -> Vec3 {
    vector - vector.project_onto(normal)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Vec2<T> {
    /// X dimension
    pub x: T,
    // Y dimension
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

impl From<Vec2<i32>> for Point {
    fn from(value: Vec2<i32>) -> Self {
        Point {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<Point> for Vec2<i32> {
    fn from(value: Point) -> Self {
        Vec2::new(value.x, value.y)
    }
}
