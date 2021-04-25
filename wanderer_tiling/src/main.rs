use doodles_lib::{
    color::Color,
    tilings::{self, TileData, WandererTile, WandererTileOrientation},
};
use nannou::prelude::*;

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 1000;
const PADDING: u32 = 50;

struct Model {
    should_update: bool,
    tiles: Vec<WandererTile>,
}

impl Model {
    fn new(should_update: bool, tiles: Vec<WandererTile>) -> Self {
        Self {
            should_update,
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
        .title("Domino Tiling")
        .resizable(false)
        .view(view)
        .key_released(key_released)
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
        5,
    );

    Model::new(true, tiles)
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(Rgb::from(Color::ChampagnePink));

    for t in &model.tiles {
        let (r, c) = match t {
            WandererTile::LeftHanded(tile_data, _) => (tile_data.rect, Color::Skobeloff),
            WandererTile::RightHanded(tile_data, _) => {
                (tile_data.rect, Color::InternationalOrangeGoldenGateBridge)
            }
        };

        draw.rect()
            .x_y(r.x(), r.y())
            .w_h(r.w(), r.h())
            .color(Rgb::from(c));
    }

    draw.to_frame(app, &frame)
        .expect("There was a problem drawing the current frame.");
}

fn key_released(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            model.should_update = !model.should_update;
        }
        _ => {}
    }
}
