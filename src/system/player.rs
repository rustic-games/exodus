//! # Input System
//!
//! Handles all input interaction from the player.

use bevy::input::{keyboard::KeyboardInput, ElementState};
use bevy::prelude::*;

use crate::kind::{CameraFocus, Direction, Position};
use crate::state::Player;

pub(crate) fn r#move(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut player: ResMut<Player>,
    mut positions: Query<&mut Position>,
) {
    let entity = player.entity;
    let position = &mut player.position;

    let mut translate = |direction: Direction| {
        let new_position = direction.to_coords().into();

        *position += new_position;
        *positions.get_mut(entity).unwrap() += new_position;
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

pub(crate) fn spawn(commands: &mut Commands, texture_atlas: Res<Handle<TextureAtlas>>) {
    let position = Position { x: 0, y: 0 };
    let entity = commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 2,
                color: Color::YELLOW,
            },
            texture_atlas: texture_atlas.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(position)
        .with(CameraFocus)
        .current_entity();

    if let Some(entity) = entity {
        commands.insert_resource(Player { entity, position });
    }
}
