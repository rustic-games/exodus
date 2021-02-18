//! Setup system.
//!
//! ...

use bevy::prelude::*;

pub(crate) fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprite_64x64.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(texture_atlas_handle);
    commands.spawn(OrthographicCameraBundle::new_2d());
}
