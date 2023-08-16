use bevy::prelude::*;

pub trait InsidePolygon {
    fn is_inside_of(&self, polygon: &[Vec2]) -> bool;
}

impl InsidePolygon for Vec2 {
    fn is_inside_of(&self, polygon: &[Vec2]) -> bool {
        if polygon.len() < 3 {
            return false;
        }

        let mut inside = false;
        let n = polygon.len() - 1;

        for i in 0..n {
            let current = polygon[i];
            let next = polygon[i + 1];

            // Check if the point is on the same side of the edge as the ray
            if (current.y > self.y) ^ (next.y > self.y)
                && self.x
                    < (next.x - current.x) * (self.y - current.y) / (next.y - current.y) + current.x
            {
                inside = !inside;
            }
        }
        inside
    }
}
