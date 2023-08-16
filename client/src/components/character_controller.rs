use bevy::prelude::*;

#[derive(Component, Default)]
pub struct CharacterController {
    pub direction: Vec2,
    pub target_direction: Vec2,
}
