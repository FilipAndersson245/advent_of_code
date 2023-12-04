#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Direction {
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
    #[inline]
    pub const fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::NorthEast => Direction::SouthWest,
            Direction::East => Direction::West,
            Direction::SouthEast => Direction::NorthWest,
            Direction::South => Direction::North,
            Direction::SouthWest => Direction::NorthEast,
            Direction::West => Direction::East,
            Direction::NorthWest => Direction::SouthEast,
        }
    }

    #[inline]
    pub const fn cardinals() -> [Self; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }

    #[inline]
    pub const fn eightway() -> [Self; 8] {
        [
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ]
    }

    #[inline]
    pub const fn diagonals() -> [Self; 4] {
        [
            Direction::NorthEast,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::NorthWest,
        ]
    }
}

impl TryFrom<(i32, i32)> for Direction {
    type Error = anyhow::Error;

    #[inline]
    fn try_from(value: (i32, i32)) -> Result<Self, Self::Error> {
        let direction = match value {
            (0, 1) => Direction::North,
            (1, 1) => Direction::NorthEast,
            (1, 0) => Direction::East,
            (1, -1) => Direction::SouthEast,
            (0, -1) => Direction::South,
            (-1, -1) => Direction::SouthWest,
            (-1, 0) => Direction::West,
            (-1, 1) => Direction::NorthWest,
            _ => {
                anyhow::bail!("tuple: ({},{}) is an invalid direction", value.0, value.1)
            }
        };
        Ok(direction)
    }
}

impl From<Direction> for (i32, i32) {
    #[inline]
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => (0, 1),
            Direction::NorthEast => (1, 1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, -1),
            Direction::South => (0, -1),
            Direction::SouthWest => (-1, -1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, 1),
        }
    }
}
