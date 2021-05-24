use nannou::{
    geom::{pt2, Point2, Rect},
    prelude::TAU,
};
use rand::Rng;
use std::ops::RangeInclusive;

pub trait Samplable {
    fn random_from_domain(domain: &Rect) -> Self;
    fn random_from_magnitude_range(magnitude_range: RangeInclusive<f32>) -> Self;
}

impl Samplable for Point2 {
    fn random_from_domain(domain: &Rect) -> Self {
        pt2(
            rand::thread_rng().gen_range(domain.left()..=domain.right()),
            rand::thread_rng().gen_range(domain.bottom()..=domain.top()),
        )
    }

    fn random_from_magnitude_range(magnitude_range: RangeInclusive<f32>) -> Self {
        Point2::from_angle(rand::thread_rng().gen_range(0.0..=TAU))
            .with_magnitude(rand::thread_rng().gen_range(magnitude_range))
    }
}
