use bevy::prelude::*;

pub trait RotateVec {
    fn rotate_origin(&self, angle: f32) -> Self;
    fn rotate_around(&self, angle: f32, origin: Vec2) -> Self;
}

impl RotateVec for Vec2 {
    fn rotate_origin(&self, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Vec2::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }
    fn rotate_around(&self, angle: f32, origin: Vec2) -> Self {
        (*self - origin).rotate_origin(angle) + origin
    }
}
