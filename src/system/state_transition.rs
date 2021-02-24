use bevy::app::AppExit;
use bevy::asset::LoadState;
use bevy::prelude::*;

use crate::app::AppState;
use crate::kind::{PlantData, TileSetAtlas};
use crate::tracing;

// TODO: Make this more generic.
//
// see: <https://discord.com/channels/691052431525675048/742884593551802431/813365447172423700>
#[tracing::instrument(skip(asset_server))]
pub(crate) fn loading_to_running(
    mut state: ResMut<State<AppState>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    tileset_atlas: Res<TileSetAtlas>,
    asset_server: Res<AssetServer>,
    textures: Res<Assets<TextureAtlas>>,
    plants: Res<PlantData>,
) {
    let atlas = match textures.get(&tileset_atlas.handle) {
        Some(atlas) => atlas,
        None => return,
    };

    match asset_server.get_load_state(&atlas.texture) {
        LoadState::Loaded => {}
        LoadState::Failed => app_exit_events.send(AppExit),
        _ => return,
    }

    if plants.is_empty() {
        return;
    }

    state.set_next(AppState::Running).unwrap()
}
