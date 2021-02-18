use bevy::prelude::*;

use crate::component::Position;
use crate::tile::Tile;

#[derive(Default)]
pub(crate) struct World {
    pub tiles: Vec<Vec<Tile>>,
}

pub(crate) struct Player {
    pub entity: Entity,
    pub position: Position,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Camera {
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self { zoom: 4. }
    }
}
