use bevy::prelude::*;

use crate::component::Position;

#[derive(Debug, Copy, Clone)]
pub(crate) enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
pub(crate) struct Move {
    pub entity: Entity,
    pub position: Position,
    pub direction: Direction,
    pub velocity: u8,
}
