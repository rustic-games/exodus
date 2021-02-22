//! # Input System
//!
//! Handles all input interaction from the player.

use bevy::input::{keyboard::KeyboardInput, ElementState};
use bevy::prelude::*;

use crate::app::OnStateEnterFix;
use crate::kind::{CameraFocus, Direction, Player, Position, TileSetAtlas};
use crate::state::TileSet;
use crate::tile::Tile;
use crate::tracing;

#[tracing::instrument(skip(tileset, keyboard_input_events, positions, tiles))]
pub(crate) fn input(
    tileset: Res<TileSet>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut positions: Query<&mut Position, With<Player>>,
    tiles: Query<&Tile>,
) {
    let mut translate = |direction: Direction| {
        let player_position = *positions.iter_mut().next().unwrap();
        let delta_position = direction.to_coords().into();

        let new_position = player_position + delta_position;
        info!(x = new_position.x, y = new_position.y, "player position");

        if let Some(tile) = tileset.tile_at(new_position) {
            let accessible = tiles
                .get(tile.entity)
                .map(|tile| tile.is_accessible())
                .unwrap_or_default();

            if !accessible {
                return;
            }

            *positions.iter_mut().next().unwrap() = new_position;
        }
    };

    for input in keyboard_input_events.iter() {
        if !matches!(input.state, ElementState::Pressed) {
            continue;
        }

        if let Some(code) = input.key_code {
            match code {
                KeyCode::W => translate(Direction::North),
                KeyCode::E => translate(Direction::NorthEast),
                KeyCode::D => translate(Direction::East),
                KeyCode::C => translate(Direction::SouthEast),
                KeyCode::X => translate(Direction::South),
                KeyCode::Z => translate(Direction::SouthWest),
                KeyCode::A => translate(Direction::West),
                KeyCode::Q => translate(Direction::NorthWest),
                KeyCode::S => {
                    // TODO: [S]leep
                    // (e.g. advance time without moving)
                    continue;
                }
                _ => continue,
            };
        }
    }
}

#[tracing::instrument(skip(commands, atlas))]
pub(crate) fn spawn(
    commands: &mut Commands,
    atlas: Res<TileSetAtlas>,
    mut fix: ResMut<OnStateEnterFix>,
) {
    if fix.player_spawn {
        return;
    }

    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 2,
                color: Color::YELLOW,
            },
            texture_atlas: atlas.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Position::default())
        .with(CameraFocus)
        .with(Player);

    fix.player_spawn = true;
}
