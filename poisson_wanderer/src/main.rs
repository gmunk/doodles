use doodles_lib::tilings::Rectangular;
use doodles_lib::{
    algorithms::poisson_disc::{self, PoissonDiscSampler},
    color::Color,
    tilings::{
        self,
        wanderer::{WandererTile, WandererTileOrientation},
    },
};
use nannou::prelude::*;

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 1000;
const REJECTION_LIMIT: u8 = 30;
const MINIMUM_RADIUS: f32 = 1.5;
const MAX_RADIUS: f32 = 3.0;
const PADDING: u32 = 50;
const STEPS: u8 = 3;
const RADIUS_FACTOR: f32 = 4.0;

fn create_poisson_disc_sampler(rect: Rect) -> PoissonDiscSampler {
    let r = poisson_disc::calculate_min_distance(&rect, Some(MINIMUM_RADIUS), Some(MAX_RADIUS));

    PoissonDiscSampler::new(rect, r, REJECTION_LIMIT)
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 || app.keys.down.contains(&Key::Delete) {
        let window_rect = app.window_rect();

        let canvas = Rect::from(window_rect)
            .pad(PADDING as f32)
            .middle_of(window_rect);

        let tiles = tilings::create_tiling(
            vec![WandererTile::LeftHanded(
                canvas,
                WandererTileOrientation::Bottom,
            )],
            STEPS,
        );

        draw.background().color(Rgb::from(Color::SpaceCadet));

        for tile in &tiles {
            let mut poisson_disc_sampler = create_poisson_disc_sampler(*tile.rect());

            let color = match tile {
                WandererTile::LeftHanded(_, _) => Color::Cerise,
                WandererTile::RightHanded(_, _) => Color::MintCream,
            };

            while !poisson_disc_sampler.is_finished() {
                if let Some(point) = poisson_disc_sampler.sample() {
                    draw.ellipse()
                        .x_y(point.x, point.y)
                        .radius(poisson_disc_sampler.r / RADIUS_FACTOR)
                        .color(Rgb::from(color));
                }
            }
        }
    }

    draw.to_frame(app, &frame)
        .expect("There was a problem drawing the current frame.");
}

fn main() {
    nannou::sketch(view).size(WINDOW_WIDTH, WINDOW_HEIGHT).run();
}
