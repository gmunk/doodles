use nannou::prelude::*;

const WINDOW_WIDTH: u32 = 1366;
const WINDOW_HEIGHT: u32 = 768;
const PADDING: u32 = 50;

trait Display {
    fn display(&self, draw: &Draw);
}

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

type Rgb = Srgb<u8>;

impl From<Color> for Rgb {
    fn from(c: Color) -> Self {
        let (r, g, b) = c.value();
        srgb(r, g, b)
    }
}

#[derive(Copy, Clone)]
struct TileData {
    rect: Rect,
    color: Color,
    num_subdivided: u8,
}

impl TileData {
    fn new(rect: Rect, color: Color, num_subdivided: u8) -> TileData {
        return TileData {
            rect,
            color,
            num_subdivided,
        };
    }
}

#[derive(Copy, Clone)]
enum Tile {
    Horizontal(TileData),
    Vertical(TileData),
}

impl Tile {
    fn subdivide(&self) -> Option<Self> {
        match self {
            Tile::Horizontal(tile_data) => {
                let substitute_horizontal_rect =
                    Rect::from_w_h(tile_data.rect.w() / 2.0, tile_data.rect.h() / 2.0);
                let substitute_vertical_rect =
                    Rect::from_w_h(tile_data.rect.w() / 4.0, tile_data.rect.h());

                match tile_data.num_subdivided {
                    0 => Some(Tile::Vertical(TileData::new(
                        substitute_vertical_rect.top_left_of(tile_data.rect),
                        Color::Skobeloff,
                        0,
                    ))),
                    1 => Some(Tile::Horizontal(TileData::new(
                        substitute_horizontal_rect.mid_top_of(tile_data.rect),
                        Color::InternationalOrangeGoldenGateBridge,
                        0,
                    ))),
                    2 => Some(Tile::Horizontal(TileData::new(
                        substitute_horizontal_rect.mid_bottom_of(tile_data.rect),
                        Color::InternationalOrangeGoldenGateBridge,
                        0,
                    ))),
                    3 => Some(Tile::Vertical(TileData::new(
                        substitute_vertical_rect.top_right_of(tile_data.rect),
                        Color::Skobeloff,
                        0,
                    ))),
                    _ => None,
                }
            }
            Tile::Vertical(tile_data) => {
                let substitute_horizontal_rect =
                    Rect::from_w_h(tile_data.rect.w(), tile_data.rect.h() / 4.0);
                let substitute_vertical_rect =
                    Rect::from_w_h(tile_data.rect.w() / 2.0, tile_data.rect.h() / 2.0);

                match tile_data.num_subdivided {
                    0 => Some(Tile::Horizontal(TileData::new(
                        substitute_horizontal_rect.top_left_of(tile_data.rect),
                        Color::InternationalOrangeGoldenGateBridge,
                        0,
                    ))),
                    1 => Some(Tile::Vertical(TileData::new(
                        substitute_vertical_rect.mid_left_of(tile_data.rect),
                        Color::Skobeloff,
                        0,
                    ))),
                    2 => Some(Tile::Vertical(TileData::new(
                        substitute_vertical_rect.mid_right_of(tile_data.rect),
                        Color::Skobeloff,
                        0,
                    ))),
                    3 => Some(Tile::Horizontal(TileData::new(
                        substitute_horizontal_rect.bottom_left_of(tile_data.rect),
                        Color::InternationalOrangeGoldenGateBridge,
                        0,
                    ))),
                    _ => None,
                }
            }
        }
    }

    fn increment_num_subdivided(&mut self) {
        match self {
            &mut Tile::Horizontal(ref mut tile_data) | &mut Tile::Vertical(ref mut tile_data) => {
                tile_data.num_subdivided += 1;
            }
        }
    }
}

impl Display for Tile {
    fn display(&self, draw: &Draw) {
        match *self {
            Tile::Horizontal(tile_data) | Tile::Vertical(tile_data) => draw
                .rect()
                .color(Rgb::from(tile_data.color))
                .stroke_color(Rgb::from(Color::ChampagnePink))
                .stroke_weight(1.0)
                .wh(tile_data.rect.wh())
                .xy(tile_data.rect.xy()),
        };
    }
}

struct Model {
    cur_index: usize,
    tiles: Vec<Tile>,
}

impl Model {
    fn new(cur_index: usize, tiles: Vec<Tile>) -> Self {
        Self { cur_index, tiles }
    }
}

impl Display for Model {
    fn display(&self, draw: &Draw) {
        draw.background().color(Rgb::from(Color::ChampagnePink));

        for t in &self.tiles {
            t.display(draw);
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
        .build()
        .expect("There was a problem creating the application's window.");

    let window_rect = match app.window(window_id) {
        None => panic!("Could not get the current window's rect."),
        Some(w) => w.rect().pad(PADDING as f32),
    };

    let tile_rect = Rect::from_w_h(
        (WINDOW_WIDTH - (2 * PADDING)) as f32,
        (WINDOW_HEIGHT - (2 * PADDING)) as f32,
    )
    .top_left_of(window_rect);

    let tiles = vec![Tile::Horizontal(TileData::new(
        tile_rect,
        Color::InternationalOrangeGoldenGateBridge,
        0,
    ))];

    Model::new(0, tiles)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let tile = model
        .tiles
        .get_mut(model.cur_index)
        .expect("There is nothing to subdivide");

    match tile.subdivide() {
        Some(t) => {
            tile.increment_num_subdivided();
            model.tiles.push(t)
        }
        None => model.cur_index += 1,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    model.display(&draw);

    draw.to_frame(app, &frame)
        .expect("There was a problem drawing the current frame.");
}
