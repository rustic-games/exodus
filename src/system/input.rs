//! # Input System
//!
//! Handles all input interaction from the player.

use bevy::prelude::*;

use crate::event::{Direction, Move};
use crate::state::{Camera, Player};

pub(crate) fn move_entity(
    mut transforms: Query<&mut Transform>,
    mut events: EventReader<Move>,
    camera: Res<Camera>,
) {
    let scale = 64. / camera.zoom;
    for event in events.iter() {
        let x = event.position.x as f32 * scale;
        let y = event.position.y as f32 * scale;

        let dv = event.velocity as f32 * scale;
        let (dv_x, dv_y) = match event.direction {
            Direction::Up => (0., dv),
            Direction::Down => (0., -dv),
            Direction::Right => (dv, 0.),
            Direction::Left => (-dv, 0.),
        };

        *transforms.get_mut(event.entity).unwrap() = Transform {
            translation: Vec3::new(x + dv_x, y + dv_y, 1.),
            ..Default::default()
        }
    }
}

pub(crate) fn move_player(
    mut event_move: ResMut<Events<Move>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player: ResMut<Player>,
) {
    let entity = player.entity;
    let position = player.position;

    let mut send = |direction| {
        event_move.send(Move {
            entity,
            position,
            direction,
            velocity: 1,
        })
    };

    if keyboard_input.just_pressed(KeyCode::W) {
        send(Direction::Up);
        player.position.y += 1;
    }
    if keyboard_input.just_pressed(KeyCode::A) {
        send(Direction::Left);
        player.position.x -= 1;
    }
    if keyboard_input.just_pressed(KeyCode::S) {
        send(Direction::Down);
        player.position.y -= 1;
    }
    if keyboard_input.just_pressed(KeyCode::D) {
        send(Direction::Right);
        player.position.x += 1;
    }
}
