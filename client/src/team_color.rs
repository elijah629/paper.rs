use bevy::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum TeamColor {
    Green,
}

impl Into<Color> for TeamColor {
    fn into(self) -> Color {
        match self {
            TeamColor::Green => Color::GREEN,
        }
    }
}
