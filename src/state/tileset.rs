use bevy::ecs::Entity;
use bevy::utils::HashMap;
use std::iter::FromIterator;

use crate::kind::Position;

#[derive(Default, Debug)]
pub(crate) struct TileSet {
    /// A collection of tiles.
    ///
    /// The first vector stores the tile columns, the second vector contains the
    /// actual tiles within each column.
    pub tiles: Vec<Vec<TileSetTile>>,

    /// A hash map of all entities and their position on the tileset.
    pub entities: HashMap<Entity, Position>,
}

impl TileSet {
    pub fn tile(&self, entity: Entity) -> Option<TileSetTile> {
        self.tiles
            .iter()
            .find_map(|rows| rows.iter().find(|tile| tile.entity == entity))
            .copied()
    }

    pub fn tile_at(&self, position: Position) -> Option<TileSetTile> {
        let (x, y) = position.coordinates();

        self.tiles
            .get((((self.tiles.len() - 1) / 2) as i32 + x) as usize)
            .and_then(|rows| rows.get((((rows.len() - 1) / 2) as i32 + y) as usize))
            .copied()
    }
}

/// A tile stored in a tile set, including all state related to the tile set.
#[derive(Debug, Copy, Clone)]
pub(crate) struct TileSetTile {
    /// The entity containing the `Tile` component.
    pub entity: Entity,

    /// The position of the tile within the tileset.
    ///
    /// This data can be used to quickly fetch surrounding tiles in the tileset.
    pub position: Position,
}

impl<T: IntoIterator<Item = Entity>> FromIterator<T> for TileSet {
    fn from_iter<I: IntoIterator<Item = T>>(tiles: I) -> Self {
        let tiles = tiles
            .into_iter()
            .enumerate()
            .map(|(col, tiles)| {
                tiles
                    .into_iter()
                    .enumerate()
                    .map(|(row, entity)| TileSetTile {
                        entity,
                        position: (col, row).into(),
                    })
                    .collect()
            })
            .collect();

        Self {
            tiles,
            entities: HashMap::default(),
        }
    }
}

impl From<Vec<Vec<Entity>>> for TileSet {
    fn from(tiles: Vec<Vec<Entity>>) -> Self {
        TileSet::from_iter(tiles)
    }
}
