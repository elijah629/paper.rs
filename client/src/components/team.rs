use bevy::prelude::*;

use crate::TeamColor;

#[derive(Component)]
pub struct Team {
    pub color: TeamColor,
    pub points: Vec<Vec2>,
    pub entity: Option<Entity>,
}
