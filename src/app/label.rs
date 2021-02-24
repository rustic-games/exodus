use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub(crate) enum SystemLabel {
    StateLoadingToRunning,
    DatumLoad,
    DataTypeSetup,
    TileSetSetup,
    TileSetSpawn,
    TileSetUpdate,
    PlayerSpawn,
    PlayerInput,
    CameraSpawn,
    CameraFocus,
    CameraZoom,
    UpdateTranslation,
    RenderTiles,
}
