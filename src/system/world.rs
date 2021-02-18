use bevy::prelude::*;
use bevy_rng::*;

use crate::component::*;
use crate::state::State;
use crate::tile::Tile;

pub(crate) fn spawn_world(
    commands: &mut Commands,
    mut game: ResMut<State>,
    mut rng: Local<Rng>,
    texture_atlas: Res<Handle<TextureAtlas>>,
) {
    let mut generate_tile = |x: i32, y: i32| {
        let mut tile = Tile::solid();

        let position = Position { x, y };

        let x = x as f32 * 16.0 - (50 * 16) as f32;
        let y = y as f32 * 16.0 - (50 * 16) as f32;

        let is_tree = rng.gen_ratio(1, 20);
        let (index, color) = if is_tree {
            (5, Color::GREEN)
        } else {
            (249, Color::DARK_GREEN)
        };

        // TODO: fog of war
        tile.revealed(true);
        tile.accessible(!is_tree);

        commands
            .spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite { index, color },
                texture_atlas: texture_atlas.clone(),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..Default::default()
            })
            .with(position);

        tile
    };

    game.tiles = (0..100)
        .map(|x| (0..100).map(|y| generate_tile(x, y)).collect())
        .collect();
}

pub(crate) fn spawn_player(
    commands: &mut Commands,
    mut game: ResMut<State>,
    mut rng: Local<Rng>,
    texture_atlas: Res<Handle<TextureAtlas>>,
) {
    //
}
