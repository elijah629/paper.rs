use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Line {
    pub points: Vec<Vec2>,
    pub entity: Option<Entity>,
}
