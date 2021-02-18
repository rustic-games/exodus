use crate::tile::Tile;

#[derive(Default)]
pub(crate) struct State {
    pub tiles: Vec<Vec<Tile>>,
    pub player_location: (i32, i32),
}
