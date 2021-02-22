use bevy::prelude::*;
use std::iter::FromIterator;

use crate::app::OnStateEnterFix;
use crate::kind::{Position, TileSetAtlas};
use crate::state::TileSet;
use crate::tile::Tile;
use crate::tracing;

#[tracing::instrument(skip(commands, asset_server))]
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

#[tracing::instrument(skip(commands))]
pub(crate) fn spawn(commands: &mut Commands, mut fix: ResMut<OnStateEnterFix>) {
    if fix.tileset_spawn {
        return;
    }

    // TODO: configurable
    let tile_count = 100;

    let mut tiles: Vec<Vec<Entity>> = vec![Vec::with_capacity(100); 100];
    for i in 0..tile_count {
        for _ in 0..tile_count {
            let entity = commands.spawn((Tile::solid(),)).current_entity().unwrap();
            tiles[i].push(entity);
        }
    }

    let tileset = TileSet::from_iter(tiles);
    commands.insert_resource(tileset);
    fix.tileset_spawn = true;
}

#[tracing::instrument(skip(entities))]
pub(crate) fn update(
    mut tileset: ResMut<TileSet>,
    entities: Query<(Entity, &Position), Changed<Position>>,
) {
    for (entity, position) in entities.iter() {
        tileset.entities.insert(entity, *position);
    }
}
