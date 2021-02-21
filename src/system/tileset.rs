use bevy::prelude::*;
use std::iter::FromIterator;

use crate::kind::{Position, TileSetAtlas};
use crate::state::TileSet;
use crate::tile::Tile;

pub(crate) fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_size = Vec2::splat(64.);
    let texture_handle = asset_server.load("sprite_64x64.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, sprite_size, 16, 16);
    let handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(TileSetAtlas {
        handle,
        sprite_size: sprite_size.x as u8,
    });
}

pub(crate) fn spawn(commands: &mut Commands) {
    // TODO: configurable
    let tile_count = 100;

    let mut tiles = vec![vec![]];
    for _ in 0..tile_count {
        let mut rows = vec![];
        for _ in 0..tile_count {
            let entity = commands.spawn((Tile::solid(),)).current_entity().unwrap();
            rows.push(entity);
        }
        tiles.push(rows);
    }

    let tileset = TileSet::from_iter(tiles);
    commands.insert_resource(tileset);
}

pub(crate) fn track_entities(
    mut tileset: ResMut<TileSet>,
    entities: Query<(Entity, &Position), Changed<Position>>,
) {
    for (entity, position) in entities.iter() {
        tileset.entities.insert(entity, *position);
    }
}