use nannou::{
    geom::{Point2, Rect},
    math::cgmath::MetricSpace,
    prelude::TAU,
};
use ndarray::{s, Array, Ix2};
use rand::{self, random, Rng};
use std::{
    cmp::min,
    ops::{Add, Range, RangeInclusive},
};

const N: u8 = 2;

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
    pub fn is_finished(&self) -> bool {
        self.active_points.len() == 0
    }

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

pub fn calculate_radius(rect: &Rect, start: Option<f32>) -> f32 {
    let mut rng = rand::thread_rng();

    let s = match start {
        None => 0.0,
        Some(s) => s,
    };

    rng.gen_range(s..=(rect.w() * rect.h()).log2())
}

fn convert_coordinate(coordinate: f32, from: RangeInclusive<f32>, to: Range<f32>) -> f32 {
    to.start + (coordinate - from.start()) * (to.end - to.start) / (from.end() - from.start())
}
trait Random {
    fn random_from_domain(domain: &Rect) -> Point2;
    fn random_from_magnitude_range(magnitude_range: RangeInclusive<f32>) -> Point2;
}

impl Random for Point2 {
    fn random_from_domain(domain: &Rect<f32>) -> Self {
        let mut rng = rand::thread_rng();

        Point2::from((
            rng.gen_range(domain.x.start..=domain.x.end),
            rng.gen_range(domain.y.start..=domain.y.end),
        ))
    }

    fn random_from_magnitude_range(magnitude_range: RangeInclusive<f32>) -> Self {
        let mut rng = rand::thread_rng();

        Point2::from_angle(rng.gen_range(0.0..=TAU)).with_magnitude(rng.gen_range(magnitude_range))
    }
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
        let cx = convert_coordinate(
            point.x,
            self.domain.x.start..=self.domain.x.end,
            0.0..self.domain.w(),
        );
        let cy = convert_coordinate(
            point.y,
            self.domain.y.invert().start..=self.domain.y.invert().end,
            0.0..self.domain.h(),
        );

        (
            (cx / self.cell_size as f32).floor() as usize,
            (cy / self.cell_size as f32).floor() as usize,
        )
    }
}
