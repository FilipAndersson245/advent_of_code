use std::{fmt::Display, hash::Hash};

use super::Direction;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn euclidean_distance(&self, other: &Self) -> f32 {
        f32::sqrt((self.x - other.x).pow(2) as f32) + f32::sqrt((self.y - other.y).pow(2) as f32)
    }

    pub fn chebyshev_distance(&self, other: &Self) -> u32 {
        u32::max(
            (self.x - other.x).abs() as u32,
            (self.y - other.y).abs() as u32,
        )
    }

    pub const fn manhattan_distance(&self, other: &Self) -> u32 {
        self.x_distance(other) + self.y_distance(other)
    }

    pub const fn x_distance(&self, other: &Self) -> u32 {
        (self.x - other.x).abs() as u32
    }

    pub const fn y_distance(&self, other: &Self) -> u32 {
        (self.y - other.y).abs() as u32
    }

    pub const fn diagonals(&self) -> [Position; 4] {
        [
            Position::new(self.x + 1, self.y + 1),
            Position::new(self.x + 1, self.y - 1),
            Position::new(self.x - 1, self.y - 1),
            Position::new(self.x - 1, self.y + 1),
        ]
    }

    pub const fn cardinals(&self) -> [Position; 4] {
        [
            Position::new(self.x, self.y + 1),
            Position::new(self.x + 1, self.y),
            Position::new(self.x, self.y - 1),
            Position::new(self.x - 1, self.y),
        ]
    }

    pub const fn eightway(&self) -> [Position; 8] {
        [
            Position::new(self.x, self.y + 1),
            Position::new(self.x + 1, self.y + 1),
            Position::new(self.x + 1, self.y),
            Position::new(self.x + 1, self.y - 1),
            Position::new(self.x, self.y - 1),
            Position::new(self.x - 1, self.y - 1),
            Position::new(self.x - 1, self.y),
            Position::new(self.x - 1, self.y + 1),
        ]
    }

    pub fn direction(&self, direction: &Direction) -> Position {
        let (x, y): (i32, i32) = (*direction).into();
        Position {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        [self.x, self.y].hash(state)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<Position> for (i32, i32) {
    fn from(Position { x, y }: Position) -> Self {
        (x, y)
    }
}

impl From<Position> for (u32, u32) {
    fn from(Position { x, y }: Position) -> Self {
        debug_assert!(x >= 0);
        debug_assert!(y >= 0);
        (x as u32, y as u32)
    }
}

impl From<(u32, u32)> for Position {
    fn from((x, y): (u32, u32)) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}

impl From<Position> for (usize, usize) {
    fn from(Position { x, y }: Position) -> Self {
        debug_assert!(x >= 0);
        debug_assert!(y >= 0);
        (x as usize, y as usize)
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::Grid;

    #[test]
    fn test_name() {
        let mut a: Grid<Option<i32>> = Grid::new(3, 3);
        a[(0, 0)] = Some(1);
        a[(0, 1)] = Some(2);
        a.col(0).for_each(|x| println!("{:?}", x));
    }
}
