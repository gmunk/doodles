use nannou::prelude::*;
use poisson_domino::PoissonDiscSampler;
use rand::Rng;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 640;
const REJECTION_LIMIT: u8 = 30;

struct Model {
    poisson_disc_sampler: PoissonDiscSampler,
    poisson_sampled_points: Vec<Point2>,
}

impl Model {
    fn new(poisson_disc_sampler: PoissonDiscSampler, poisson_sampled_points: Vec<Point2>) -> Self {
        Self {
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
        .title("Poisson Domino")
        .resizable(false)
        .view(view)
        .build()
        .expect("There was a problem creating the application's window.");

    let _window_rect = match app.window(window_id) {
        None => panic!("Could not get the current window's rect."),
        Some(w) => w.rect(),
    };

    let mut rng = rand::thread_rng();

    let working_rect = Rect::from_w_h(600.0f32, 300.0f32);

    let r = rng.gen_range(0.0..(working_rect.w() * working_rect.h()).log2());

    let poisson_disc_sampler = PoissonDiscSampler::new(working_rect, r, REJECTION_LIMIT);

    let poisson_sampled_points: Vec<Point2> = vec![];

    Model::new(poisson_disc_sampler, poisson_sampled_points)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if !model.poisson_disc_sampler.is_finished() {
        match model.poisson_disc_sampler.sample() {
            None => {}
            Some(p) => model.poisson_sampled_points.push(p),
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for p in &model.poisson_sampled_points {
        draw.ellipse()
            .xy(*p)
            .w_h(5.0, 5.0)
            .stroke_color(WHITE)
            .stroke_weight(1.0)
            .no_fill();
    }

    draw.to_frame(app, &frame)
        .expect("There was a problem drawing the current frame.");
}
