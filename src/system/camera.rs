use bevy::prelude::*;
use bevy::render::camera::Camera;

use crate::app::OnStateEnterFix;
use crate::kind::{CameraFocus, Position};
use crate::tracing;

#[tracing::instrument(skip(commands))]
pub(crate) fn spawn(commands: &mut Commands, mut fix: ResMut<OnStateEnterFix>) {
    if fix.camera_spawn {
        return;
    }

    commands
        .spawn(OrthographicCameraBundle::new_2d())
        .with(Position::default());

    fix.camera_spawn = true;
}

#[tracing::instrument(skip(query))]
pub(crate) fn zoom(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Camera>>) {
    enum ZoomDirection {
        In,
        Out,
    }

    let mut direction = None;
    if input.pressed(KeyCode::Caret) {
        direction = if input.pressed(KeyCode::LShift) {
            Some(ZoomDirection::Out)
        } else {
            Some(ZoomDirection::In)
        }
    }

    if let Some(direction) = direction {
        let step = match direction {
            ZoomDirection::In => -0.02,
            ZoomDirection::Out => 0.02,
        };

        for mut transform in query.iter_mut() {
            let scale = &mut transform.scale;
            let new = (scale.x + step).clamp(1., 4.);

            scale.x = new;
            scale.y = new;
        }
    }
}

#[tracing::instrument(skip(camera, target))]
pub(crate) fn focus(
    mut camera: Query<&mut Position, With<Camera>>,
    target: Query<&Position, (With<CameraFocus>, Changed<Position>)>,
) {
    for target_position in target.iter() {
        for mut camera_position in camera.iter_mut() {
            *camera_position = *target_position;
        }
    }
}
