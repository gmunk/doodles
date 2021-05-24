use doodles_lib::collections::Initializer;
use doodles_lib::flowfield::{Flowfield, Noise};
use doodles_lib::particle::Particle;
use doodles_lib::rand::Samplable;
use doodles_lib::{
    color::Color,
    poisson_disc::{self, PoissonDiscSampler},
};
use nannou::geom::Ellipse;
use nannou::math::cgmath::MetricSpace;
use nannou::noise::{Perlin, Seedable};
use nannou::prelude::*;
use rand::Rng;

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 1000;
const REJECTION_LIMIT: u8 = 30;
const MINIMUM_RADIUS: f32 = 3.0;
const MAXIMUM_RADIUS: f32 = 7.0;

fn create_poisson_disc_sampler(rect: Rect) -> PoissonDiscSampler {
    let r = poisson_disc::calculate_min_distance(&rect, Some(MINIMUM_RADIUS), Some(MAXIMUM_RADIUS));

    PoissonDiscSampler::new(rect, r, REJECTION_LIMIT)
}

struct Point {
    x: f32,
    y: f32,
    r: f32,
}

struct Model {
    canvas: Ellipse,
    flowfield: Flowfield<Perlin>,
    particles: Vec<Particle>,
    poissonfield: Vec<Point>,
    should_draw: bool,
}

impl Model {
    fn new(
        canvas: Ellipse,
        flowfield: Flowfield<Perlin>,
        particles: Vec<Particle>,
        poissonfield: Vec<Point>,
        should_draw: bool,
    ) -> Self {
        Self {
            canvas,
            flowfield,
            particles,
            poissonfield,
            should_draw,
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
        .key_pressed(key_pressed)
        .build()
        .expect("There was a problem creating the application's window.");

    let window_rect = match app.window(window_id) {
        None => panic!("Could not get the current window's rect."),
        Some(w) => w.rect(),
    };

    let mut rng = rand::thread_rng();

    let ellipse = Ellipse::new(Rect::from_xy_wh(pt2(0.0, 0.0), vec2(600.0, 600.0)), 4);

    let subdivisions = ellipse.rect.subdivisions();

    let flowfield_canvas =
        Rect::from_corners(subdivisions[2].top_left(), subdivisions[0].bottom_right());
    let poissonfield_canvas =
        Rect::from_corners(subdivisions[3].top_left(), subdivisions[1].bottom_right());

    let noise = Noise::new(Perlin::new().set_seed(rng.gen()), 0.0, 0.5, 0.0005);
    let flowfield = Flowfield::new(flowfield_canvas, noise, 20);
    let particles = Vec::initialize(10000, |_| {
        Particle::new(
            Point2::random_from_domain(&flowfield_canvas),
            None,
            Vector2::zero(),
            Vector2::from_angle(rand::thread_rng().gen_range(0.0..=TAU)),
            1.5,
        )
    });

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

    Model::new(ellipse, flowfield, particles, poissonfield, true)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.flowfield.update();

    for particle in &mut model.particles {
        let vector = match model.flowfield.get_vector_at(&particle.position) {
            None => panic!(
                "No flowfield vector was found for particle at position: {}/{}.",
                particle.position.x, particle.position.y
            ),
            Some(vector) => vector,
        };
        particle.apply_force(vector);
        particle.update();
        particle.wrap_around(&model.flowfield.canvas);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 || app.keys.down.contains(&Key::Delete) {
        draw.background().color(rgb8(33, 37, 41));

        for point in &model.poissonfield {
            draw.ellipse()
                .xy(pt2(point.x, point.y))
                .radius(point.r)
                .color(rgb8(173, 181, 189));
        }
    }
    if model.should_draw {
        for particle in &model.particles {
            if particle.position.distance(model.canvas.rect.xy()) <= 300.0 {
                particle.display(&draw);
            }
        }
    }

    draw.to_frame(app, &frame)
        .expect("There was a problem drawing the current frame.");
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => app.main_window().capture_frame(format!(
            "{}.png",
            app.exe_name()
                .expect("There was a problem getting the running executable's name.")
        )),
        Key::Space => model.should_draw = !model.should_draw,
        _ => {}
    }
}

fn main() {
    nannou::app(model).update(update).run();
}
