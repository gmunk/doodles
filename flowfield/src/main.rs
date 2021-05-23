use nannou::noise::{NoiseFn, Perlin, Seedable};
use nannou::prelude::*;
use rand::prelude::*;

const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 800;

trait Initializer<T, U, F> {
    fn initialize(count: usize, f: F) -> T;
}

impl<T, U, F> Initializer<T, U, F> for T
where
    T: std::iter::FromIterator<U>,
    F: FnMut(usize) -> U,
{
    fn initialize(count: usize, f: F) -> T {
        (0..count).map(f).collect::<T>()
    }
}

struct Noise<T>
where
    T: Seedable + NoiseFn<nannou::noise::Point3<f64>>,
{
    generator: T,
    z_offset: f64,
    xy_increment: f64,
    z_increment: f64,
}

impl<T> Noise<T>
where
    T: Seedable + NoiseFn<nannou::noise::Point3<f64>>,
{
    fn new(generator: T, z_offset: f64, xy_increment: f64, z_increment: f64) -> Self {
        Self {
            generator,
            z_offset,
            xy_increment,
            z_increment,
        }
    }
}

struct Flowfield<T>
where
    T: Seedable + NoiseFn<nannou::noise::Point3<f64>>,
{
    rows: u32,
    columns: u32,
    resolution: u32,
    canvas: Rect,
    vectors: Vec<Vector2>,
    noise: Noise<T>,
}

impl<T> Flowfield<T>
where
    T: Seedable + NoiseFn<nannou::noise::Point3<f64>>,
{
    fn new(canvas: Rect, noise: Noise<T>, resolution: u32) -> Self {
        let rows = (canvas.h() / resolution as f32).floor() as u32 + 1;
        let columns = (canvas.w() / resolution as f32).floor() as u32 + 1;

        let vectors = vec![Vector2::zero(); (rows * columns) as usize];

        Self {
            rows,
            columns,
            resolution,
            canvas,
            vectors,
            noise,
        }
    }

    fn update(&mut self) {
        let mut x_offset = 0.0;

        for row in 0..self.rows {
            let mut y_offset = 0.0;

            for column in 0..self.columns {
                self.vectors[(column + row * self.columns) as usize] = Vector2::from_angle(
                    (self
                        .noise
                        .generator
                        .get([x_offset, y_offset, self.noise.z_offset])
                        * TAU_F64) as f32,
                );
                y_offset += self.noise.xy_increment
            }

            x_offset += self.noise.xy_increment;
        }

        self.noise.z_offset += self.noise.z_increment;
    }

    fn display(&self, draw: &Draw) {
        for row in 0..self.rows {
            for column in 0..self.columns {
                let x = (self.canvas.left() + (self.resolution as f32 / 2.0))
                    + (self.resolution * column) as f32;
                let y = (self.canvas.top() - (self.resolution as f32 / 2.0))
                    - (self.resolution * row) as f32;

                draw.translate(vec3(x, y, 0.0))
                    .line()
                    .rotate(self.vectors[(column + row * self.columns) as usize].angle())
                    .color(rgb8(0, 0, 0))
                    .weight(1.0)
                    .points(pt2(0.0, 0.0), pt2(self.resolution as f32 / 2.0, 0.0));
            }
        }
    }

    fn get_vector_at(&self, point: &Point2) -> Option<&Vector2> {
        let mapped_x = map_range(
            point.x,
            self.canvas.left(),
            self.canvas.right(),
            0.0,
            self.canvas.w(),
        );
        let mapped_y = map_range(
            point.y,
            self.canvas.bottom(),
            self.canvas.top(),
            0.0,
            self.canvas.h(),
        );

        let x = (mapped_x / self.resolution as f32).floor();
        let y = (mapped_y / self.resolution as f32).floor();

        self.vectors.get((x + y * self.columns as f32) as usize)
    }
}

struct Particle {
    position: Point2,
    previous_position: Option<Point2>,
    velocity: Vector2,
    acceleration: Vector2,
    velocity_limit: f32,
}

impl Particle {
    fn new(
        position: Point2,
        previous_position: Option<Point2>,
        velocity: Vector2,
        acceleration: Vector2,
        velocity_limit: f32,
    ) -> Self {
        Self {
            position,
            previous_position,
            velocity,
            acceleration,
            velocity_limit,
        }
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.limit_magnitude(self.velocity_limit);
        self.previous_position = Some(self.position);
        self.position += self.velocity;
        self.acceleration *= 0.0;
    }

    fn display(&self, draw: &Draw) {
        if let Some(previous_position) = self.previous_position {
            draw.line()
                .color(rgba8(1, 22, 39, 25))
                .weight(1.0)
                .points(previous_position, self.position);
        }
    }

    fn wrap_around(&mut self, canvas: &Rect) {
        if self.position.x > canvas.right() {
            self.position.x = canvas.left();
            self.previous_position = Some(self.position);
        }

        if self.position.x < canvas.left() {
            self.position.x = canvas.right();
            self.previous_position = Some(self.position);
        }

        if self.position.y > canvas.top() {
            self.position.y = canvas.bottom();
            self.previous_position = Some(self.position);
        }

        if self.position.y < canvas.bottom() {
            self.position.y = canvas.top();
            self.previous_position = Some(self.position);
        }
    }

    fn apply_force(&mut self, force: &Vector2) {
        self.acceleration += *force;
    }
}

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
    let window_id = app
        .new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Flowfield")
        .resizable(false)
        .view(view)
        .build()
        .expect("There was a problem creating the application's window.");

    let window_rect = match app.window(window_id) {
        None => panic!("Could not get the current window's rect."),
        Some(w) => w.rect(),
    };
    let mut rng = rand::thread_rng();

    let canvas = Rect::from_xy_wh(pt2(0.0, 0.0), vec2(600.0, 400.0));

    let noise = Noise::new(Perlin::new().set_seed(rng.gen()), 0.0, 0.05, 0.0005);

    let flowfield = Flowfield::new(canvas, noise, 20);

    let particles = Vec::initialize(10000, |_| {
        Particle::new(
            pt2(
                rng.gen_range(canvas.left()..canvas.right()),
                rng.gen_range(canvas.bottom()..canvas.top()),
            ),
            None,
            Vector2::zero(),
            Vector2::from_angle(rand::thread_rng().gen_range(0.0..=TAU)),
            2.0,
        )
    });

    Model::new(flowfield, particles)
}

fn update(app: &App, model: &mut Model, _update: Update) {
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
