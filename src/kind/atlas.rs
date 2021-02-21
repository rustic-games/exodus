use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

pub(crate) struct TileSetAtlas {
    pub handle: Handle<TextureAtlas>,
    pub sprite_size: u8,
}

#[derive(Copy, Clone)]
pub(crate) enum TileSprite {
    Tree,
    Grass,
}

impl TileSprite {
    pub fn index(self) -> u32 {
        use TileSprite::*;

        match self {
            Tree => 5,
            Grass => 249,
        }
    }
}

impl Deref for TileSetAtlas {
    type Target = Handle<TextureAtlas>;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl DerefMut for TileSetAtlas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.handle
    }
}
