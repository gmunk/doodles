//! Provides an implementation of Bridson's poisson-disc sampling algorithm.
//! The module exposes a struct which holds the algorithm parameters and provides methods
//! for step-by-step (point-by-point) sampling.
use crate::rand::Samplable;
use nannou::{
    geom::{Point2, Rect},
    math::{cgmath::MetricSpace, map_range},
};
use ndarray::{s, Array, Ix2};
use rand::{self, random, Rng};
use std::{cmp::min, ops::Add};

/// The number of dimensions in which the algorithm works.
/// For doodling purposes this is set to 2.
/// In other words no 3D or more poisson-disc sampled doodles are planed, for now.
const N: u8 = 2;

/// Encapsulates parameters and functionality related to Birdson's poisson-disc sampling algorithm.
/// The sampler expects several pieces of data–minimum distance r, maximum number of tries for a correct
/// point sample, a grid of cells,
/// where each point is going to be assigned and an empty list of active points.
pub struct PoissonDiscSampler {
    pub r: f32,
    k: u8,
    grid: Grid,
    active_points: Vec<Point2>,
}

impl PoissonDiscSampler {
    pub fn new(domain: Rect, r: f32, k: u8) -> Self {
        let cell_size = (r / (N as f32).sqrt()).floor() as u32;

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

            let p = Point2::random_from_magnitude_range(self.r..=2.0 * self.r);

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

    /// Checks if the poisson-disc sampling is finished,
    /// i.e. if the provided sample domain has been filled with points.
    /// In terms of the implementation this means that the method checks if the active points list
    /// is empty.
    pub fn is_finished(&self) -> bool {
        self.active_points.len() == 0
    }

    /// Checks if a point is a valid sample.
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
                    Some(p) => !(p.distance(*point) < self.r),
                }) {
                    true => SampleStatus::Valid,
                    false => SampleStatus::Invalid,
                }
            }
            false => SampleStatus::Invalid,
        }
    }
}

/// Calculates the minimum distance, r, based on the sample domain's size.
/// This function works with two optional values–start and end of the range.
///
/// If start is provided, the function just uses it, if it is not, the lower bound for generating
/// the minimum distance becomes 0.
///
/// If end is not provided, the function calculates it like so lof2(w*h), using the sample domain's
/// width and height. If end is provided, no calculation is done and the function uses it for the
/// upper bound of the minimum distance.
pub fn calculate_min_distance(rect: &Rect, start: Option<f32>, end: Option<f32>) -> f32 {
    let mut rng = rand::thread_rng();

    let s = match start {
        None => 0.0,
        Some(s) => s,
    };

    let e = match end {
        None => (rect.w() * rect.h()).log2(),
        Some(e) => e,
    };

    rng.gen_range(s..=e)
}

enum SampleStatus {
    Valid,
    Invalid,
}

struct Grid {
    cell_size: u32,
    domain: Rect,
    internal_array: Array<Option<Point2>, Ix2>,
}

impl Grid {
    fn new(cell_size: u32, domain: Rect) -> Self {
        let w = (domain.w() / (cell_size as f32)).ceil();
        let h = (domain.h() / (cell_size as f32)).ceil();

        let internal_array =
            Array::<Option<Point2>, Ix2>::from_elem((w as usize, h as usize), None);

        Self {
            cell_size,
            domain,
            internal_array,
        }
    }

    fn insert(&mut self, point: Point2) {
        let (x_index, y_index) = self.calculate_grid_indices(&point);

        self.internal_array
            .slice_mut(s![x_index as usize, y_index as usize])
            .fill(Some(point));
    }

    fn calculate_grid_indices(&self, point: &Point2) -> (usize, usize) {
        let cx = map_range(
            point.x,
            self.domain.x.start,
            self.domain.x.end,
            0.0,
            self.domain.w(),
        );
        let cy = map_range(
            point.y,
            self.domain.y.start,
            self.domain.y.end,
            0.0,
            self.domain.h(),
        );

        (
            (cx / self.cell_size as f32).floor() as usize,
            (cy / self.cell_size as f32).floor() as usize,
        )
    }
}
