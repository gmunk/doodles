use doodles_lib::{
    color::Color,
    poisson_disc::{self, PoissonDiscSampler},
    tilings::{self, TileData, WandererTile, WandererTileOrientation},
};
use nannou::prelude::*;
use std::collections::VecDeque;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const REJECTION_LIMIT: u8 = 30;
const MINIMUM_RADIUS: f32 = 2.0;
const MAX_RADIUS: f32 = 3.0;
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
    current_tile: Option<WandererTile>,
    tiles: VecDeque<WandererTile>,
}

impl Model {
    fn new(
        poisson_disc_sampler: PoissonDiscSampler,
        poisson_sampled_points: Vec<Point>,
        current_tile: Option<WandererTile>,
        tiles: VecDeque<WandererTile>,
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

    let mut tiles = VecDeque::from(tilings::create_tiling(
        WandererTile::LeftHanded(TileData::new(canvas_rect), WandererTileOrientation::Bottom),
        5,
    ));

    let tile = tiles.pop_front().expect("Nothing to pop");
    let tile_rect = get_tile_rect(&tile);

    let poisson_disc_sampler = create_poisson_disc_sampler(tile_rect);

    Model::new(poisson_disc_sampler, vec![], Some(tile), tiles)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if let Some(current_tile) = &model.current_tile {
        let color = match current_tile {
            WandererTile::LeftHanded(_, _) => Color::Cerise,
            WandererTile::RightHanded(_, _) => Color::MintCream,
        };

        if let Some(p) = model.poisson_disc_sampler.sample() {
            model.poisson_sampled_points.push(Point {
                x: p.x,
                y: p.y,
                r: model.poisson_disc_sampler.r / 4.0,
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
    let r = poisson_disc::calculate_radius(&rect, Some(MINIMUM_RADIUS), Some(MAX_RADIUS));

    PoissonDiscSampler::new(rect, r, REJECTION_LIMIT)
}

fn get_tile_rect(tile: &WandererTile) -> Rect {
    match tile {
        WandererTile::LeftHanded(tile_data, _) | WandererTile::RightHanded(tile_data, _) => {
            tile_data.rect
        }
    }
}
