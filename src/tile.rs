#[derive(Default)]
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

    pub fn is_revealed(&self) -> bool {
        self.revealed
    }

    pub fn accessible(&mut self, accessible: bool) {
        self.accessible = accessible;
    }

    pub fn is_accessible(&self) -> bool {
        self.accessible
    }
}

/// The type of matter that makes up a tile.
pub(crate) enum Matter {
    /// A solid tile.
    ///
    /// Can be traversed by the player (unless blocked), and broken down (unless
    /// indestructible).
    Solid,

    /// A liquid tile.
    ///
    /// Can be traversed by swimming (if applicable), or contained in
    /// containers.
    Liquid,

    /// A gas tile.
    ///
    /// Usually means the player is falling, or flying.
    Gas,

    /// A vacuum tile.
    ///
    /// A tile devoid of any matter. Requires special gear to traverse.
    Vacuum,
}

impl Default for Matter {
    fn default() -> Self {
        Self::Solid
    }
}
