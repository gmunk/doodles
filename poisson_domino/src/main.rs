use doodles_lib::{
    color::Color,
    poisson_disc::{self, PoissonDiscSampler},
    tilings::{self, DominoTile},
};
use nannou::prelude::*;
use std::collections::VecDeque;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const REJECTION_LIMIT: u8 = 30;
const MINIMUM_RADIUS: f32 = 3.0;
const PADDING: u32 = 50;

struct Point {
    x: f32,
    y: f32,
    r: f32,
    color: Color,
}

struct Model {
    poisson_disc_sampler: PoissonDiscSampler,
    poisson_sampled_points: Vec<Point>,
    current_tile: Option<DominoTile>,
    tiles: VecDeque<DominoTile>,
}

impl Model {
    fn new(
        poisson_disc_sampler: PoissonDiscSampler,
        poisson_sampled_points: Vec<Point>,
        current_tile: Option<DominoTile>,
        tiles: VecDeque<DominoTile>,
    ) -> Self {
        Self {
            poisson_disc_sampler,
            poisson_sampled_points,
            current_tile,
            tiles,
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

    let window_rect = match app.window(window_id) {
        None => panic!("Could not get the current window's rect."),
        Some(w) => w.rect().pad(PADDING as f32),
    };

    let canvas_rect = Rect::from_w_h(
        (WINDOW_WIDTH - (2 * PADDING)) as f32,
        (WINDOW_HEIGHT - (2 * PADDING)) as f32,
    )
    .top_left_of(window_rect);

    let mut tiles = VecDeque::from(tilings::create_domino_tiling(canvas_rect, 4));
    let tile = tiles.pop_front().expect("Nothing to pop");
    let tile_rect = get_tile_rect(&tile);

    let poisson_disc_sampler = create_poisson_disc_sampler(tile_rect);

    let poisson_sampled_points: Vec<Point> = vec![];

    Model::new(
        poisson_disc_sampler,
        poisson_sampled_points,
        Some(tile),
        tiles,
    )
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if let Some(current_tile) = &model.current_tile {
        let color = match current_tile {
            DominoTile::Horizontal(_) => Color::Red032,
            DominoTile::Vertical(_) => Color::Navy2380,
        };

        if let Some(p) = model.poisson_disc_sampler.sample() {
            model.poisson_sampled_points.push(Point {
                x: p.x,
                y: p.y,
                r: model.poisson_disc_sampler.r,
                color,
            });
        }

        if model.poisson_disc_sampler.is_finished() {
            match model.tiles.pop_front() {
                None => model.current_tile = None,
                Some(t) => {
                    model.poisson_disc_sampler = create_poisson_disc_sampler(get_tile_rect(&t));
                    model.current_tile = Some(t);
                }
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(Rgb::from(Color::Yellow47));

    for p in &model.poisson_sampled_points {
        draw.ellipse()
            .x_y(p.x, p.y)
            .w_h(p.r / 2.0, p.r / 2.0)
            .color(Rgb::from(p.color));
    }

    draw.to_frame(app, &frame)
        .expect("There was a problem drawing the current frame.");
}

fn create_poisson_disc_sampler(rect: Rect) -> PoissonDiscSampler {
    let r = poisson_disc::calculate_radius(&rect, Some(MINIMUM_RADIUS));

    PoissonDiscSampler::new(rect, r, REJECTION_LIMIT)
}

fn get_tile_rect(tile: &DominoTile) -> Rect {
    match tile {
        DominoTile::Horizontal(ref tile_data) | DominoTile::Vertical(ref tile_data) => {
            tile_data.rect
        }
    }
}
