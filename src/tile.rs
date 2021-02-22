#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Tile {
    revealed: bool,
    accessible: bool,

    matter: Matter,
}

impl Tile {
    pub fn solid() -> Self {
        Self {
            matter: Matter::Solid,
            ..Default::default()
        }
    }

    pub fn revealed(&mut self, revealed: bool) {
        self.revealed = revealed;
    }

    pub fn accessible(&mut self, accessible: bool) {
        self.accessible = accessible;
    }
}

/// The type of matter that makes up a tile.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Matter {
    /// A solid tile.
    ///
    /// Can be traversed by the player (unless blocked), and broken down (unless
    /// indestructible).
    Solid,
}

impl Default for Matter {
    fn default() -> Self {
        Self::Solid
    }
}
