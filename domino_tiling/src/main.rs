use doodles_lib::tilings::{self, DominoTile};
use nannou::prelude::*;

const WINDOW_WIDTH: u32 = 1366;
const WINDOW_HEIGHT: u32 = 768;
const PADDING: u32 = 50;

type Rgb = Srgb<u8>;

#[derive(Copy, Clone)]
enum Color {
    Skobeloff,
    ChampagnePink,
    InternationalOrangeGoldenGateBridge,
}

impl Color {
    fn value(&self) -> (u8, u8, u8) {
        match self {
            Color::Skobeloff => (25u8, 114u8, 120u8),
            Color::ChampagnePink => (237u8, 221u8, 212u8),
            Color::InternationalOrangeGoldenGateBridge => (196u8, 69u8, 54u8),
        }
    }
}

impl From<Color> for Rgb {
    fn from(c: Color) -> Self {
        let (r, g, b) = c.value();
        srgb(r, g, b)
    }
}

struct Model {
    should_update: bool,
    tiles: Vec<DominoTile>,
}

impl Model {
    fn new(should_update: bool, tiles: Vec<DominoTile>) -> Self {
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

    let tiles = tilings::create_domino_tiling(canvas_rect, 2);

    Model::new(true, tiles)
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(Rgb::from(Color::ChampagnePink));

    for t in &model.tiles {
        let (r, c) = match t {
            DominoTile::Horizontal(tile_data) => (tile_data.rect, Color::Skobeloff),
            DominoTile::Vertical(tile_data) => {
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
