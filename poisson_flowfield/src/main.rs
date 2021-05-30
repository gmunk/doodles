use doodles_lib::{
    algorithms::{
        flowfield::{Flowfield, Noise},
        poisson_disc::{self, PoissonDiscSampler},
    },
    collections::Initializer,
    particle::Particle,
    rand::Samplable,
};
use nannou::{
    geom::Ellipse,
    math::MetricSpace,
    noise::{Perlin, Seedable},
    prelude::*,
};
use rand::Rng;

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 1000;
const REJECTION_LIMIT: u8 = 30;
const MINIMUM_RADIUS: f32 = 3.0;
const MAXIMUM_RADIUS: f32 = 7.0;
const CANVAS_RADIUS: f32 = 300.0;
const NOISE_Z_OFFSET: f64 = 0.0;
const NOISE_Z_INCREMENT: f64 = 0.0005;
const NOISE_XY_INCREMENT: f64 = 0.5;
const FLOWFIELD_RESOLUTION: u32 = 20;
const NUMBER_PARTICLES: usize = 10000;
const RADIUS_FACTOR: f32 = 4.0;

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
    should_draw_particles: bool,
    debug: bool,
}

impl Model {
    fn new(
        canvas: Ellipse,
        flowfield: Flowfield<Perlin>,
        particles: Vec<Particle>,
        poissonfield: Vec<Point>,
        should_draw_particles: bool,
        debug: bool,
    ) -> Self {
        Self {
            canvas,
            flowfield,
            particles,
            poissonfield,
            should_draw_particles,
            debug,
        }
    }
}

fn model(app: &App) -> Model {
    let _window_id = app
        .new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Poisson Flowfield")
        .resizable(false)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .expect("There was a problem creating the application's window.");

    let mut rng = rand::thread_rng();

    let ellipse = Ellipse::new(
        Rect::from_xy_wh(
            pt2(0.0, 0.0),
            vec2(CANVAS_RADIUS * 2.0, CANVAS_RADIUS * 2.0),
        ),
        4,
    );

    let subdivisions = ellipse.rect.subdivisions();

    let flowfield_canvas =
        Rect::from_corners(subdivisions[2].top_left(), subdivisions[0].bottom_right());
    let poissonfield_canvas =
        Rect::from_corners(subdivisions[3].top_left(), subdivisions[1].bottom_right());

    let noise = Noise::new(
        Perlin::new().set_seed(rng.gen()),
        NOISE_Z_OFFSET,
        NOISE_XY_INCREMENT,
        NOISE_Z_INCREMENT,
    );
    let flowfield = Flowfield::new(flowfield_canvas, noise, FLOWFIELD_RESOLUTION);
    let particles = Vec::initialize(NUMBER_PARTICLES, |_| {
        Particle::new(
            Point2::random_from_domain(&flowfield_canvas),
            None,
            Vector2::zero(),
            Vector2::from_angle(rand::thread_rng().gen_range(0.0..=TAU)),
            1.5,
            rgba8(173, 181, 189, 25),
        )
    });

    let mut poisson_disc_sampler = create_poisson_disc_sampler(poissonfield_canvas);
    let mut poissonfield = vec![];

    while !poisson_disc_sampler.is_finished() {
        if let Some(p) = poisson_disc_sampler.sample() {
            if p.distance(ellipse.rect.xy()) <= CANVAS_RADIUS {
                poissonfield.push(Point {
                    x: p.x,
                    y: p.y,
                    r: poisson_disc_sampler.r / RADIUS_FACTOR,
                });
            }
        }
    }

    Model::new(ellipse, flowfield, particles, poissonfield, true, false)
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

    if model.debug {
        model.flowfield.display(&draw);
    }

    if model.should_draw_particles {
        for particle in &model.particles {
            if particle.position.distance(model.canvas.rect.xy()) <= CANVAS_RADIUS {
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
        Key::Space => model.should_draw_particles = !model.should_draw_particles,
        _ => {}
    }
}

fn main() {
    nannou::app(model).update(update).run();
}
