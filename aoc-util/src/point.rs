use std::{
    fmt::Display,
    hash::Hash,
    ops::{Add, Sub},
};

use crate::direction::Direction;

/// A point in a 2D grid.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn euclidean_distance(&self, other: &Self) -> f32 {
        f32::sqrt((self.x - other.x).pow(2) as f32) + f32::sqrt((self.y - other.y).pow(2) as f32)
    }

    #[inline]
    /// Chebyshev distance, also known as "Maximum distance" or "Chessboard distance
    pub fn chebyshev_distance(&self, other: &Self) -> u32 {
        u32::max(
            (self.x - other.x).abs() as u32,
            (self.y - other.y).abs() as u32,
        )
    }

    #[inline]
    /// Manhattan distance, also known as "city block distance"
    pub const fn manhattan_distance(&self, other: &Self) -> u32 {
        self.x_distance(other) + self.y_distance(other)
    }

    #[inline]
    pub const fn x_distance(&self, other: &Self) -> u32 {
        (self.x - other.x).abs() as u32
    }

    #[inline]
    pub const fn y_distance(&self, other: &Self) -> u32 {
        (self.y - other.y).abs() as u32
    }

    #[inline]
    pub const fn diagonals(&self) -> [Point; 4] {
        [
            Point::new(self.x + 1, self.y + 1),
            Point::new(self.x + 1, self.y - 1),
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x - 1, self.y + 1),
        ]
    }

    #[inline]
    pub const fn cardinals(&self) -> [Point; 4] {
        [
            Point::new(self.x, self.y + 1),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y - 1),
            Point::new(self.x - 1, self.y),
        ]
    }

    #[inline]
    pub const fn eightway(&self) -> [Point; 8] {
        [
            Point::new(self.x, self.y + 1),
            Point::new(self.x + 1, self.y + 1),
            Point::new(self.x + 1, self.y),
            Point::new(self.x + 1, self.y - 1),
            Point::new(self.x, self.y - 1),
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x - 1, self.y),
            Point::new(self.x - 1, self.y + 1),
        ]
    }

    #[inline]
    pub fn step(self, direction: Direction, step_size: u32) -> Self {
        let (x, y) = direction.into();
        Self {
            x: self.x + x * step_size as i32,
            y: self.y + y * step_size as i32,
        }
    }
}

impl Hash for Point {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        [self.x, self.y].hash(state)
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Direction) -> Self::Output {
        let (x, y) = rhs.into();
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Direction> for Point {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Direction) -> Self::Output {
        let (x, y) = rhs.into();
        Self {
            x: self.x - x,
            y: self.y - y,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(i32, i32)> for Point {
    #[inline]
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<Point> for (i32, i32) {
    #[inline]
    fn from(Point { x, y }: Point) -> Self {
        (x, y)
    }
}

impl From<Point> for (u32, u32) {
    #[inline]
    fn from(Point { x, y }: Point) -> Self {
        debug_assert!(x >= 0);
        debug_assert!(y >= 0);
        (x as u32, y as u32)
    }
}

impl From<(u32, u32)> for Point {
    #[inline]
    fn from((x, y): (u32, u32)) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}

impl From<Point> for (usize, usize) {
    #[inline]
    fn from(Point { x, y }: Point) -> Self {
        debug_assert!(x >= 0);
        debug_assert!(y >= 0);
        (x as usize, y as usize)
    }
}

impl From<(usize, usize)> for Point {
    #[inline]
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::direction::Direction;

    #[test]
    fn test_direction() {
        let p = Point::new(0, 0);
        let c = p + Direction::North;
        assert_eq!(c.x, 0);
        assert_eq!(c.y, 1);
    }
}
