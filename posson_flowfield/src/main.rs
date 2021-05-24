use doodles_lib::{
    color::Color,
    poisson_disc::{self, PoissonDiscSampler},
};
use nannou::geom::Ellipse;
use nannou::math::cgmath::MetricSpace;
use nannou::prelude::*;

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 1000;
const REJECTION_LIMIT: u8 = 30;
const MINIMUM_RADIUS: f32 = 3.0;
const MAXIMUM_RADIUS: f32 = 7.0;

struct Point {
    x: f32,
    y: f32,
    r: f32,
}

struct Model {
    canvas: Ellipse,
    flowfield_canvas: Rect,
    poissonfield: Vec<Point>,
}

impl Model {
    fn new(canvas: Ellipse, flowfield_canvas: Rect, poissonfield: Vec<Point>) -> Self {
        Self {
            canvas,
            flowfield_canvas,
            poissonfield,
        }
    }
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Poisson Flowfield")
        .resizable(false)
        .view(view)
        .build()
        .expect("There was a problem creating the application's window.");

    let window_rect = match app.window(window_id) {
        None => panic!("Could not get the current window's rect."),
        Some(w) => w.rect(),
    };

    let ellipse = Ellipse::new(Rect::from_xy_wh(pt2(0.0, 0.0), vec2(600.0, 600.0)), 4);

    let subdivisions = ellipse.rect.subdivisions();

    let flowfield_canvas =
        Rect::from_corners(subdivisions[2].top_left(), subdivisions[0].bottom_right());
    let poissonfield_canvas =
        Rect::from_corners(subdivisions[3].top_left(), subdivisions[1].bottom_right());

    let mut poisson_disc_sampler = create_poisson_disc_sampler(poissonfield_canvas);
    let mut poissonfield = vec![];

    while !poisson_disc_sampler.is_finished() {
        if let Some(p) = poisson_disc_sampler.sample() {
            if p.distance(ellipse.rect.xy()) <= 300.0 {
                poissonfield.push(Point {
                    x: p.x,
                    y: p.y,
                    r: poisson_disc_sampler.r / 4.0,
                });
            }
        }
    }

    Model::new(ellipse, flowfield_canvas, poissonfield)
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 || app.keys.down.contains(&Key::Delete) {
        draw.background().color(rgb8(126, 189, 194));

        for point in &model.poissonfield {
            draw.ellipse()
                .xy(pt2(point.x, point.y))
                .radius(point.r)
                .color(rgb8(1, 22, 39));
        }
    }

    draw.to_frame(app, &frame)
        .expect("There was a problem drawing the current frame.");
}

fn main() {
    nannou::app(model).run();
}

fn create_poisson_disc_sampler(rect: Rect) -> PoissonDiscSampler {
    let r = poisson_disc::calculate_min_distance(&rect, Some(MINIMUM_RADIUS), Some(MAXIMUM_RADIUS));

    PoissonDiscSampler::new(rect, r, REJECTION_LIMIT)
}
