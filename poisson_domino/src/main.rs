use doodles_lib::tilings::Rectangular;
use doodles_lib::{
    algorithms::poisson_disc::{self, PoissonDiscSampler},
    color::Color,
    tilings::{self, domino::DominoTile},
};
use nannou::prelude::*;
use std::collections::VecDeque;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const REJECTION_LIMIT: u8 = 30;
const MINIMUM_RADIUS: f32 = 3.0;
const PADDING: u32 = 50;
const TILES_PADDING: f32 = 10.0;
const RADIUS_FACTOR: f32 = 4.0;

fn create_poisson_disc_sampler(rect: Rect) -> PoissonDiscSampler {
    let r = poisson_disc::calculate_min_distance(&rect, Some(MINIMUM_RADIUS), None);

    PoissonDiscSampler::new(rect, r, REJECTION_LIMIT)
}

fn pick_current_color(tile: &DominoTile) -> Color {
    match tile {
        DominoTile::Horizontal(_) => Color::RedPigment,
        DominoTile::Vertical(_) => Color::MintCream,
    }
}

struct Model {
    poisson_disc_sampler: PoissonDiscSampler,
    current_tile: Option<DominoTile>,
    current_point: Option<Point2>,
    current_color: Color,
    tiles: VecDeque<DominoTile>,
}

impl Model {
    fn new(
        poisson_disc_sampler: PoissonDiscSampler,
        current_tile: Option<DominoTile>,
        current_point: Option<Point2>,
        current_color: Color,
        tiles: VecDeque<DominoTile>,
    ) -> Self {
        Self {
            poisson_disc_sampler,
            current_tile,
            current_point,
            current_color,
            tiles,
        }
    }
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Poisson Domino")
        .resizable(false)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .expect("There was a problem creating the application's window.");

    let window_rect = match app.window(window_id) {
        None => panic!("Could not get the current window's rect."),
        Some(w) => w.rect(),
    };

    let canvas_rect = Rect::from(window_rect)
        .pad(PADDING as f32)
        .middle_of(window_rect);

    let mut tiles = VecDeque::from(tilings::create_tiling(
        vec![DominoTile::Horizontal(canvas_rect)],
        2,
    ));
    let tile = tiles.pop_front().expect("Nothing to pop");

    let poisson_disc_sampler = create_poisson_disc_sampler(tile.rect().pad(TILES_PADDING));

    let color = pick_current_color(&tile);

    Model::new(poisson_disc_sampler, Some(tile), None, color, tiles)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if let Some(_) = &model.current_tile {
        if let Some(point) = model.poisson_disc_sampler.sample() {
            model.current_point = Some(point);
        }

        if model.poisson_disc_sampler.is_finished() {
            match model.tiles.pop_front() {
                None => {
                    model.current_tile = None;
                }
                Some(t) => {
                    model.poisson_disc_sampler =
                        create_poisson_disc_sampler(t.rect().pad(TILES_PADDING));
                    model.current_color = pick_current_color(&t);
                    model.current_tile = Some(t);
                    model.current_point = None;
                }
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 || app.keys.down.contains(&Key::Delete) {
        draw.background().color(Rgb::from(Color::EerieBlack));
    }

    if let Some(current_point) = &model.current_point {
        draw.ellipse()
            .x_y(current_point.x, current_point.y)
            .radius(model.poisson_disc_sampler.r / RADIUS_FACTOR)
            .color(Rgb::from(model.current_color));
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

fn main() {
    nannou::app(model).update(update).run();
}
