use nannou::geom::Ellipse;
use nannou::math::cgmath::num_traits::Pow;
use nannou::prelude::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::Rng;

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 1000;
const N: u32 = 5000;

struct Model {
    circle: Circle,
    c: f32,
    a0: f32,
    squares: Vec<Square>,
    counter: u32,
}

impl Model {
    fn new(circle: Circle, c: f32, a0: f32, squares: Vec<Square>) -> Self {
        Self {
            circle,
            c,
            a0,
            squares,
            counter: 1,
        }
    }
}

struct Circle {
    radius: f32,
    center: Point2,
}

impl Circle {
    fn new(radius: f32, center: Point2) -> Self {
        Self { radius, center }
    }
}

struct Square {
    rect: Rect,
    color: Rgb8,
}

impl Square {
    fn is_valid(&self, others: &[Square]) -> bool {
        others.iter().all(|s| match s.rect.overlap(self.rect) {
            None => true,
            Some(_) => false,
        })
    }

    fn from_xy_area_color(point: Point2<f32>, area: f32, color: Rgb8) -> Self {
        let dimensions = Vector2::from([area.sqrt(); 2]);

        let rect = Rect::from_xy_wh(point, dimensions);

        Self { rect, color }
    }
}

struct Point(Point2<f32>);

impl Point {
    fn random_within_circle(circle: &Circle) -> Self {
        let rr = circle.radius * rand::thread_rng().gen::<f32>().sqrt();
        let theta = 2.0 * PI * rand::thread_rng().gen::<f32>();

        Point(Point2::new(
            circle.center.x + rr * theta.cos(),
            circle.center.y + rr * theta.sin(),
        ))
    }
}

fn riemann_zeta(n: f32, c: f32) -> f32 {
    n.powf(-c)
}

fn calculate_new_area(a0: f32, n: f32, c: f32) -> f32 {
    a0 * riemann_zeta(n, c)
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Space Packing")
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

    let palette = [(rgb8(1, 22, 39), 8), (rgb8(217, 3, 104), 2)];
    let palette_distribution = WeightedIndex::new(palette.iter().map(|color| color.1)).unwrap();

    let circle = Circle::new(300f32, Point2::new(0.0, 0.0));

    let c = rng.gen_range(1.1..1.4);

    let riemann_zeta_sum: f32 = (1..=N).map(|n| riemann_zeta(n as f32, c)).sum();

    let a0 = (PI * circle.radius.pow(2)) / riemann_zeta_sum;

    let mut squares = vec![];

    for i in 1..=N {
        let area = calculate_new_area(a0, i as f32, c);

        let square = loop {
            let Point(point) = Point::random_within_circle(&circle);

            let square = Square::from_xy_area_color(
                point,
                area,
                palette[palette_distribution.sample(&mut rng)].0,
            );

            if square.is_valid(&squares) {
                break square;
            }
        };

        squares.push(square);
    }

    Model::new(circle, c, a0, squares)
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(rgb8(126, 189, 194));

    for square in &model.squares {
        draw.rect()
            .x_y(square.rect.x(), square.rect.y())
            .w_h(square.rect.w(), square.rect.h())
            .color(square.color);
    }

    draw.to_frame(app, &frame)
        .expect("There was a problem drawing the current frame.");
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    println!("Key pressed");

    match key {
        Key::S => app.main_window().capture_frame(format!(
            "{}.png",
            app.exe_name()
                .expect("There was a problem getting the running executable's name.")
        )),
        _ => {}
    }
}

fn main() {
    nannou::app(model).run();
}
