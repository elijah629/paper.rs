use bevy::prelude::*;

use super::intersects::intersects;

pub fn has_self_intersection(line: &[Vec2]) -> bool {
    if line.len() < 1 {
        return false;
    }

    for i in 0..line.len() - 1 {
        for j in i..line.len() - 1 {
            let a = (line[i], line[i + 1]);
            let b = (line[j], line[j + 1]);

            if a == b {
                continue;
            }

            if intersects(a.0, a.1, b.0, b.1) {
                return true;
            }
        }
    }
    false
}
