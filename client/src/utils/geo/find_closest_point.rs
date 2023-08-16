use bevy::prelude::*;

use crate::utils::unwrap_abort::UnwrapAbort;

pub fn find_closest_point(point: Vec2, points: &[Vec2]) -> usize {
    points
        .into_iter()
        .enumerate()
        .map(|(i, x)| (i, x.distance_squared(point)))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_abort())
        .unwrap_abort()
        .0
}
