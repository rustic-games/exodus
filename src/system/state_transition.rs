use bevy::app::AppExit;
use bevy::asset::LoadState;
use bevy::prelude::*;

use crate::app::AppState;
use crate::kind::TileSetAtlas;

pub(crate) fn loading_to_running(
    mut state: ResMut<State<AppState>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    tileset_atlas: Res<TileSetAtlas>,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<TextureAtlas>>,
) {
    let atlas = match assets.get(&tileset_atlas.handle) {
        Some(atlas) => atlas,
        None => return,
    };

    match asset_server.get_load_state(&atlas.texture) {
        LoadState::Loaded => state.set_next(AppState::Running).unwrap(),
        LoadState::Failed => app_exit_events.send(AppExit),
        _ => {}
    }
}
