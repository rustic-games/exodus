use bevy::prelude::*;
use bevy_rng::*;

use crate::kind::{Position, TileSetAtlas, TileSprite};
use crate::state::TileSet;
use crate::tile::Tile;
use crate::tracing;

/// For any entity for which its tileset position has changed, update its
/// viewport translation property to match the new position.
#[tracing::instrument(skip(target))]
pub(crate) fn update_translation(
    atlas: Res<TileSetAtlas>,
    mut target: Query<(&Position, &mut Transform), Changed<Position>>,
) {
    let tile_size = atlas.sprite_size as i32;

    for (position, mut transform) in target.iter_mut() {
        transform.translation.x = (position.x * tile_size) as f32;
        transform.translation.y = (position.y * tile_size) as f32;
    }
}

#[tracing::instrument(skip(commands, tiles))]
pub(crate) fn tiles(
    commands: &mut Commands,
    atlas: Res<TileSetAtlas>,
    tileset: Res<TileSet>,
    mut tiles: Query<(Entity, &mut Tile), Changed<Tile>>,

    // TODO: remove, see note below
    mut rng: Local<Rng>,
) {
    let tile_size = atlas.sprite_size as i32;

    for (entity, mut tile) in tiles.iter_mut() {
        let tileset_tile = match tileset.tile(entity) {
            Some(tile) => tile,
            None => {
                warn!("tried to fetch non-existing tile in tileset");
                return;
            }
        };

        commands.set_current_entity(entity);

        // TODO:
        //
        // Tiles themselves don't represent any "thing".
        //
        // Take for example a tree, it is its own entity, and has its own
        // logic and sprite.
        //
        // Similarly, a tile doesn't need to have its ground covered (think
        // grass, dirt), because some tiles might not be accessible (think
        // tunnels underground, outside of the tunnels there's "nothing",
        // because you can't actually walk there unless you excavate that
        // location first).
        //
        // For simplicity, right now we treat each tile as having either a tree
        // or a ground but at some point this will have to be refactored.

        let is_tree = rng.gen_ratio(1, 20);
        let (index, color) = if is_tree {
            (TileSprite::Tree.index(), Color::GREEN)
        } else {
            (TileSprite::Grass.index(), Color::DARK_GREEN)
        };

        // TODO: fog of war
        tile.revealed(true);
        tile.accessible(!is_tree);

        let size = (tileset.tiles.len() / 2) as i32;

        let x = tileset_tile.position.x * tile_size - size * tile_size;
        let y = tileset_tile.position.y * tile_size - size * tile_size;

        commands.with_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite { index, color },
            texture_atlas: atlas.clone(),
            transform: Transform {
                translation: Vec3::new(x as f32, y as f32, 0.),
                scale: Vec3::one(),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
