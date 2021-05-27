//! This module contains implementations of tiling algorithms.
//!
//! In mathematics a tiling is a collection of geometric shapes (called tiles) which cover
//! the plane without any gaps or overlaps. Each tile of a tilling must be a topological disc,
//! meaning it must be a connected piece without any holes or lines.
pub mod domino;
pub mod wanderer;

/// This Divisible trait facilitates the division of a tile.
///
/// Each divisible tile must implement the divide method which returns a vec of new tiles, the
/// successors of the one being divided.
pub trait Divisible {
    fn divide(&self) -> Vec<Self>
    where
        Self: Sized;
}

/// Create a tiling based on the type of element that the input vec holds.
///
/// To create a tiling one must supply a vec holding the initial tiles (usually just one)
/// and an [`u8`] representing how many steps the tiling algorithm should take.
/// Based on the type that the vec holds, an appropriate algorithm will be executed and a new vec,
/// holding the tiles of the completed tiling, will be returned.
pub fn create_tiling<T>(mut tiles: Vec<T>, mut steps: u8) -> Vec<T>
where
    T: Divisible,
{
    steps -= 1;

    let divided_tiles: Vec<T> = tiles
        .drain(..)
        .map(|tile| tile.divide())
        .flatten()
        .collect();

    if steps == 0 {
        divided_tiles
    } else {
        create_tiling(divided_tiles, steps)
    }
}
