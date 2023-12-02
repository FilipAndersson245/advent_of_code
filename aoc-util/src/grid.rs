use std::ops::{Index, IndexMut};

use crate::Point;

/// A grid is a 2D array of data. rows are first, then columns.
#[derive(Clone)]
pub struct Grid<T>
where
    T: Clone + Default,
{
    data: Vec<T>,
    pub width: u32,
    pub height: u32,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            data: std::iter::repeat(T::default())
                .take((width * height) as usize)
                .collect(),
            width,
            height,
        }
    }

    pub fn row(&self, row: u32) -> impl Iterator<Item = &T> {
        self.data
            .iter()
            .skip((row * self.width) as usize)
            .take(self.width as usize)
    }

    pub fn col(&self, col: u32) -> impl Iterator<Item = &T> {
        self.data
            .iter()
            .skip(col as usize)
            .step_by(self.width as usize)
    }
}

impl<T: Clone + Default> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        debug_assert!(index.x >= 0);
        debug_assert!(index.y >= 0);
        self.data
            .get((index.y as u32 * self.width) as usize + index.x as usize)
            .unwrap()
    }
}

impl<T> IndexMut<Point> for Grid<T>
where
    T: Clone + Default,
{
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        debug_assert!(index.x >= 0);
        debug_assert!(index.y >= 0);
        self.data
            .get_mut((index.y as u32 * self.width) as usize + index.x as usize)
            .unwrap()
    }
}

impl<T> Index<(i32, i32)> for Grid<T>
where
    T: Clone + Default,
{
    type Output = T;

    #[inline]
    fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
        &self[Point::new(x, y)]
    }
}

impl<T> IndexMut<(i32, i32)> for Grid<T>
where
    T: Clone + Default,
{
    #[inline]
    fn index_mut(&mut self, (x, y): (i32, i32)) -> &mut Self::Output {
        &mut self[Point::new(x, y)]
    }
}

impl<T> Index<(u32, u32)> for Grid<T>
where
    T: Clone + Default,
{
    type Output = T;

    #[inline]
    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        &self[Point::new(x as i32, y as i32)]
    }
}

impl<T> IndexMut<(u32, u32)> for Grid<T>
where
    T: Clone + Default,
{
    #[inline]
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Self::Output {
        &mut self[Point::new(x as i32, y as i32)]
    }
}
