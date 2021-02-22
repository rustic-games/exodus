use bevy::math::IVec2;
use std::ops::{Add, AddAssign};

#[derive(Default, Debug, Copy, Clone)]
pub(crate) struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn coordinates(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        (x as i32, y as i32).into()
    }
}

impl From<Position> for IVec2 {
    fn from(position: Position) -> Self {
        Self::new(position.x, position.y)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
