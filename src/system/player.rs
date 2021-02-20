//! # Input System
//!
//! Handles all input interaction from the player.

use bevy::input::{keyboard::KeyboardInput, ElementState};
use bevy::prelude::*;

use crate::kind::{CameraFocus, Direction, Position};
use crate::state::{Camera, Player, World};

pub(crate) fn r#move(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut player: ResMut<Player>,
    mut transforms: Query<&mut Transform>,
    world: Res<World>,
    camera: Res<Camera>,
) {
    let entity = player.entity;
    let position = &mut player.position;
    let scale = world.tileset.tile_size / camera.zoom;

    let mut translate = |direction: Direction| {
        let (dv_x, dv_y) = direction.to_coords();
        let x = position.x as f32 * scale;
        let y = position.y as f32 * scale;

        *position += (dv_x, dv_y).into();

        *transforms.get_mut(entity).unwrap().translation =
            *Vec3::new(x + (dv_x as f32 * scale), y + (dv_y as f32 * scale), 1.);
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

pub(crate) fn spawn(
    commands: &mut Commands,
    texture_atlas: Res<Handle<TextureAtlas>>,
    camera: Res<Camera>,
) {
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
                scale: Vec3::new(1. / camera.zoom, 1. / camera.zoom, 1.),
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
