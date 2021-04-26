use doodles_lib::{
    color::Color,
    poisson_disc::{self, PoissonDiscSampler},
    tilings::{self, TileData, WandererTile, WandererTileOrientation},
};
use nannou::prelude::*;

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 1000;
const REJECTION_LIMIT: u8 = 30;
const MINIMUM_RADIUS: f32 = 1.5;
const MAX_RADIUS: f32 = 2.0;
const PADDING: u32 = 50;

struct Point {
    x: f32,
    y: f32,
    r: f32,
    color: Color,
}

struct Model {
    poisson_sampled_points: Vec<Point>,
}

impl Model {
    fn new(poisson_sampled_points: Vec<Point>) -> Self {
        Self {
            poisson_sampled_points,
        }
    }
}

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Poisson Wanderer")
        .resizable(false)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .expect("There was a problem creating the application's window.");

    let window_rect = match app.window(window_id) {
        None => panic!("Could not get the current window's rect."),
        Some(w) => w.rect().pad(PADDING as f32),
    };

    let canvas_rect = Rect::from_w_h(
        (WINDOW_WIDTH - (2 * PADDING)) as f32,
        (WINDOW_HEIGHT - (2 * PADDING)) as f32,
    )
    .top_left_of(window_rect);

    let tiles = tilings::create_tiling(
        WandererTile::LeftHanded(TileData::new(canvas_rect), WandererTileOrientation::Bottom),
        7,
    );

    let mut poisson_sampled_points: Vec<Point> = vec![];

    for t in &tiles {
        let t_rect = get_tile_rect(t);
        let mut poisson_disc_sampler = create_poisson_disc_sampler(t_rect);

        let color = match t {
            WandererTile::LeftHanded(_, _) => Color::Cerise,
            WandererTile::RightHanded(_, _) => Color::MintCream,
        };

        while !poisson_disc_sampler.is_finished() {
            if let Some(p) = poisson_disc_sampler.sample() {
                poisson_sampled_points.push(Point {
                    x: p.x,
                    y: p.y,
                    r: poisson_disc_sampler.r / 4.0,
                    color,
                });
            }
        }
    }

    Model::new(poisson_sampled_points)
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(Rgb::from(Color::SpaceCadet));

    for p in &model.poisson_sampled_points {
        draw.ellipse()
            .x_y(p.x, p.y)
            .radius(p.r)
            .color(Rgb::from(p.color));
    }

    draw.to_frame(app, &frame)
        .expect("There was a problem drawing the current frame.");
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    match key {
        Key::S => app.main_window().capture_frame(format!(
            "{}.png",
            app.exe_name()
                .expect("There was a problem getting the running executable's name.")
        )),
        _ => {}
    }
}

fn create_poisson_disc_sampler(rect: Rect) -> PoissonDiscSampler {
    let r = poisson_disc::calculate_min_distance(&rect, Some(MINIMUM_RADIUS), Some(MAX_RADIUS));

    PoissonDiscSampler::new(rect, r, REJECTION_LIMIT)
}

fn get_tile_rect(tile: &WandererTile) -> Rect {
    match tile {
        WandererTile::LeftHanded(tile_data, _) | WandererTile::RightHanded(tile_data, _) => {
            tile_data.rect
        }
    }
}
