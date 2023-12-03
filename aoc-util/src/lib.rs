pub mod direction;
pub mod grid;
pub mod point;

pub use direction::Direction;
pub use grid::Grid;
pub use point::Point;

#[cfg(feature = "parsing")]
pub use parsing;

#[cfg(feature = "gxhash")]
pub use gxhash::{GxHashMap as HashMap, GxHashSet as HashSet};

#[cfg(not(feature = "gxhash"))]
pub use std::collections::{HashMap, HashSet};

// pub type IndexMap<T, V> = indexmap::IndexMap<T, V, gxhash::GxHasher>;

pub mod prelude {
    pub use crate::{Direction, Grid, HashMap, HashSet, Point};
}
