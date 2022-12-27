//! Collection of various utility functions.

use bevy::prelude::Vec3;
use bevy_turborand::{rng::Rng, TurboRand};
use bracket_pathfinding::prelude::Point;
use std::ops::{Range, RangeBounds};

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
pub(crate) fn lerp_range(value: f32, range: Range<f32>) -> f32 {
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

// impl<i32> Vec2<i32> {
//     pub fn random(rng: &mut Rng, x_bounds: Range<i32>, y_bounds: Range<i32>) -> Self {
//         Vec2 { x: rng.i32(x_bounds), y: rng.i32(y_bounds) }
//     }
// }

impl Into<Point> for Vec2<i32> {
    fn into(self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<Point> for Vec2<i16> {
    fn into(self) -> Point {
        Point {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

impl Into<Point> for Vec2<i8> {
    fn into(self) -> Point {
        Point {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

impl Into<Vec2<i32>> for Point {
    fn into(self) -> Vec2<i32> {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}
