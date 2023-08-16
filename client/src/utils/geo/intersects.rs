use bevy::prelude::*;

pub fn intersects(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> bool {
    let d1 = orientation(b1, b2, a1);
    let d2 = orientation(b1, b2, a2);
    let d3 = orientation(a1, a2, b1);
    let d4 = orientation(a1, a2, b2);

    // Check if the line segments intersect
    (d1 > 0.0 && d2 < 0.0 || d1 < 0.0 && d2 > 0.0) && (d3 > 0.0 && d4 < 0.0 || d3 < 0.0 && d4 > 0.0)
}

pub fn orientation(p1: Vec2, p2: Vec2, p3: Vec2) -> f32 {
    (p3.x - p1.x) * (p2.y - p1.y) - (p2.x - p1.x) * (p3.y - p1.y)
}
