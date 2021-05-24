use doodles_lib::{
    collections::Initializer,
    flowfield::{Flowfield, Noise},
    particle::Particle,
    rand::Samplable,
};
use nannou::{
    noise::{Perlin, Seedable},
    prelude::*,
};
use rand::prelude::*;

const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 800;

struct Model {
    flowfield: Flowfield<Perlin>,
    particles: Vec<Particle>,
}

impl Model {
    fn new(flowfield: Flowfield<Perlin>, particles: Vec<Particle>) -> Self {
        Self {
            flowfield,
            particles,
        }
    }
}

fn model(app: &App) -> Model {
    let _window_id = app
        .new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Flowfield")
        .resizable(false)
        .view(view)
        .build()
        .expect("There was a problem creating the application's window.");

    let mut rng = rand::thread_rng();

    let canvas = Rect::from_xy_wh(pt2(0.0, 0.0), vec2(600.0, 400.0));

    let noise = Noise::new(Perlin::new().set_seed(rng.gen()), 0.0, 0.05, 0.0005);

    let flowfield = Flowfield::new(canvas, noise, 20);

    let particles = Vec::initialize(10000, |_| {
        Particle::new(
            Point2::random_from_domain(&canvas),
            None,
            Vector2::zero(),
            Vector2::from_angle(rand::thread_rng().gen_range(0.0..=TAU)),
            2.0,
        )
    });

    Model::new(flowfield, particles)
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
        draw.background().color(rgb8(126, 189, 194));
    }

    for particle in &model.particles {
        particle.display(&draw);
    }

    draw.to_frame(app, &frame)
        .expect("There was a problem drawing the current frame.");
}

fn main() {
    nannou::app(model).update(update).run();
}
