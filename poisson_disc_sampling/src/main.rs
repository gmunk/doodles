use doodles_lib::poisson_disc::{self, PoissonDiscSampler};
use nannou::prelude::*;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 640;
const REJECTION_LIMIT: u8 = 30;
const MINIMUM_RADIUS: f32 = 3.0;

struct Model {
    r: f32,
    poisson_disc_sampler: PoissonDiscSampler,
    poisson_sampled_points: Vec<Point2>,
}

impl Model {
    fn new(
        r: f32,
        poisson_disc_sampler: PoissonDiscSampler,
        poisson_sampled_points: Vec<Point2>,
    ) -> Self {
        Self {
            r,
            poisson_disc_sampler,
            poisson_sampled_points,
        }
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Poisson-disc Sampling")
        .resizable(false)
        .view(view)
        .build()
        .expect("There was a problem creating the application's window.");

    let window_rect = match app.window(window_id) {
        None => panic!("Could not get the current window's rect."),
        Some(w) => w.rect(),
    };

    let r = poisson_disc::calculate_min_distance(&window_rect, Some(MINIMUM_RADIUS), None);

    let poisson_disc_sampler = PoissonDiscSampler::new(window_rect, r, REJECTION_LIMIT);

    let poisson_sampled_points: Vec<Point2> = vec![];

    Model::new(r, poisson_disc_sampler, poisson_sampled_points)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if !model.poisson_disc_sampler.is_finished() {
        if let Some(p) = model.poisson_disc_sampler.sample() {
            model.poisson_sampled_points.push(p);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for p in &model.poisson_sampled_points {
        draw.ellipse()
            .xy(*p)
            .w_h(model.r / 2.0, model.r / 2.0)
            .stroke_color(WHITE)
            .stroke_weight(1.0)
            .no_fill();
    }

    draw.to_frame(app, &frame)
        .expect("There was a problem drawing the current frame.");
}
