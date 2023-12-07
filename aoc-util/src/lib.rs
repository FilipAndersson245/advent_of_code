pub mod direction;
pub mod grid;
pub mod point;

pub use direction::Direction;
pub use grid::Grid;
pub use point::Point;

#[cfg(feature = "parsing")]
pub use parsing;

pub type HashMap<K, V> = std::collections::HashMap<K, V, gxhash::GxHasher>;
pub type HashSet<T> = std::collections::HashSet<T, gxhash::GxHasher>;

pub type IndexMap<T, V> = indexmap::IndexMap<T, V, gxhash::GxHasher>;
pub type IndexSet<T> = indexmap::IndexSet<T, gxhash::GxHasher>;

pub mod prelude {
    pub use crate::{Direction, Grid, HashMap, HashSet, Point};
}
