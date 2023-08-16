use bevy::{prelude::*, utils::HashMap};

use crate::TeamColor;

#[derive(Resource, Default)]
pub struct Game {
    pub team_mats: HashMap<TeamColor, Handle<ColorMaterial>>,
}
