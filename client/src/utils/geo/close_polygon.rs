use bevy::prelude::*;

pub fn close_polygon(points: &[Vec2]) -> Vec<Vec2> {
    let mut points = points.to_vec();
    points.push(points[0]);
    points
}
