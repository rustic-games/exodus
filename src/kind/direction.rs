#[derive(Debug, Copy, Clone)]
pub(crate) enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn to_coords(self) -> (i32, i32) {
        use self::Direction::*;

        match self {
            North => (0, 1),
            NorthEast => (1, 1),
            East => (1, 0),
            SouthEast => (1, -1),
            South => (0, -1),
            SouthWest => (-1, -1),
            West => (-1, 0),
            NorthWest => (-1, 1),
        }
    }
}
