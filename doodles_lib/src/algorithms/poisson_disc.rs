//! Provides an implementation of Bridson's poisson-disc sampling algorithm.
//!
//! This module exposes a struct, [`PoissonDiscSampler`], which holds the algorithm parameters
//! and provides methods for step-by-step (point-by-point) sampling.
use crate::{geometry::coordinates, rand::Samplable};
use nannou::{
    geom::{Point2, Rect},
    math::MetricSpace,
};
use ndarray::{s, Array, Ix2};
use rand::{self, random, Rng};
use std::{cmp::min, ops::Add};

const N: u8 = 2;

/// Calculates the minimum distance between each sample (point) for a [`PoissonDiscSampler`].
///
/// This function is used as an utility, the caller supplies two values,
/// which together comprise a range from which a value must be picked. This value is
/// the parameter "r" used by every instance of [`PoissonDiscSampler`].
///
/// The range used for the random generation is [start..end].
pub fn calculate_min_distance(rect: &Rect, start: Option<f32>, end: Option<f32>) -> f32 {
    let s = match start {
        None => 0.0,
        Some(s) => s,
    };

    let e = match end {
        None => (rect.w() * rect.h()).log2(),
        Some(e) => e,
    };

    rand::thread_rng().gen_range(s..=e)
}

/// Represents a grid on top of the domain (plane).
///
/// Each cell of the grid can contain only one point and the purpose is to speed up the checks
/// whether a brand new point violates the requirement that the distance between it and all
/// other points must be greater than or equal to "r".
struct Grid {
    cell_size: f32,
    domain: Rect,
    internal_array: Array<Option<Point2>, Ix2>,
}

impl Grid {
    fn new(cell_size: f32, domain: Rect) -> Self {
        let w = (domain.w() / cell_size).ceil() as usize;
        let h = (domain.h() / cell_size).ceil() as usize;

        let internal_array = Array::<Option<Point2>, Ix2>::from_elem((w, h), None);

        Self {
            cell_size,
            domain,
            internal_array,
        }
    }

    /// Inserts a new point in the grid.
    ///
    /// Calculates the grid indices of the position in the grid,
    /// based on the screen coordinates of the point which is to be inserted.
    fn insert(&mut self, point: Point2) {
        let (x_index, y_index) = self.calculate_grid_indices(&point);

        self.internal_array
            .slice_mut(s![x_index, y_index])
            .fill(Some(point));
    }

    fn calculate_grid_indices(&self, point: &Point2) -> (usize, usize) {
        let converted_point = coordinates::convert_to_upper_left_origin(point, &self.domain);

        (
            (converted_point.x / self.cell_size).floor() as usize,
            (converted_point.y / self.cell_size).floor() as usize,
        )
    }
}

enum SampleStatus {
    Valid,
    Invalid,
}

/// Encapsulates data and functionality related to Birdson's poisson-disc sampling algorithm.
///
/// The sampler expects several pieces of data–minimum distance r,
/// maximum number of tries to find a valid point sample and a grid of cells,
/// where each point is going to be placed and an empty list of active points.
pub struct PoissonDiscSampler {
    pub r: f32,
    k: u8,
    grid: Grid,
    active_points: Vec<Point2>,
}

impl PoissonDiscSampler {
    /// Constructs a new instance of [`PoissonDiscSampler`].
    pub fn new(domain: Rect, r: f32, k: u8) -> Self {
        let cell_size = (r / (N as f32).sqrt()).floor();

        let mut grid = Grid::new(cell_size, domain);
        let mut active_points: Vec<Point2> = vec![];

        let p = Point2::random_from_domain(&domain);

        grid.insert(p);
        active_points.push(p);

        Self {
            r,
            k,
            grid,
            active_points,
        }
    }

    /// Samples a new point by getting a random active point and generating a sample candidate
    /// positioned somewhere in the spherical annulus between r and 2r.
    ///
    /// It then proceed to check the surrounding neighbourhood of points, trying to determine
    /// if the new point in as far away as required (distance shouldn't be less than r)
    /// from every single neighbour.
    ///
    /// It the point is not a valid sample, the active point is removed from the active points list.
    ///
    /// Returns the new point if it is a valid sample or None if it is not.
    pub fn sample(&mut self) -> Option<Point2> {
        let index = (random::<f32>() * self.active_points.len() as f32).floor() as usize;

        let active_point = self.active_points[index];

        let mut counter: u8 = 0;

        let new_point = loop {
            counter += 1;

            let p = Point2::random_from_magnitude_range(self.r..=(2.0 * self.r));

            let new_point = active_point.add(p);

            match self.check_point(&new_point) {
                SampleStatus::Valid => break Some(new_point),
                SampleStatus::Invalid => {
                    if counter == self.k {
                        break None;
                    }
                }
            }
        };

        match new_point {
            None => {
                self.active_points.remove(index);
                None
            }
            Some(p) => {
                self.grid.insert(p);
                self.active_points.push(p);
                Some(p)
            }
        }
    }

    /// Checks if the poisson-disc sampling is finished.
    ///
    /// Poisson-disc sampling has finished when the provided sample domain has
    /// been filled with points In terms o implementation this means that the method checks
    /// if the active points list is empty.
    pub fn is_finished(&self) -> bool {
        self.active_points.len() == 0
    }

    /// Checks if a point is a valid sample.
    ///
    /// The method creates a window (neighbourhood) of cells around the new point's cell.
    /// It then checks each cell in this windows for two things, whether it doesn't contains a point
    /// or if the containing point is sufficiently far away from the new one.
    fn check_point(&self, point: &Point2) -> SampleStatus {
        match self.grid.domain.contains(*point) {
            true => {
                let (x_index, y_index) = self.grid.calculate_grid_indices(point);

                let shape = self.grid.internal_array.shape();

                let x_start = match x_index.checked_sub(1) {
                    None => 0usize,
                    Some(x) => x,
                };
                let x_end = min(x_index + 1, shape[0] - 1);

                let y_start = match y_index.checked_sub(1) {
                    None => 0usize,
                    Some(y) => y,
                };
                let y_end = min(y_index + 1, shape[1] - 1);

                let neighbours = self
                    .grid
                    .internal_array
                    .slice(s![x_start..=x_end, y_start..=y_end]);

                match neighbours.iter().all(|&p| match p {
                    None => true,
                    Some(p) => p.distance(*point) >= self.r,
                }) {
                    true => SampleStatus::Valid,
                    false => SampleStatus::Invalid,
                }
            }
            false => SampleStatus::Invalid,
        }
    }
}
