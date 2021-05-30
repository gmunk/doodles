use doodles_lib::{
    color::Color,
    tilings::{
        self,
        wanderer::{WandererTile, WandererTileOrientation},
    },
};
use nannou::prelude::*;

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 1000;
const PADDING: u32 = 50;
const TILES_PADDING: u32 = 1;
const STEPS: u8 = 4;

fn main() {
    nannou::sketch(view).size(WINDOW_WIDTH, WINDOW_HEIGHT).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 || app.keys.down.contains(&Key::Delete) {
        let window_rect = app.window_rect();

        let canvas = Rect::from(window_rect)
            .pad(PADDING as f32)
            .middle_of(window_rect);

        let tiles = tilings::create_tiling(
            vec![WandererTile::LeftHanded(
                canvas,
                WandererTileOrientation::Bottom,
            )],
            STEPS,
        );

        for t in &tiles {
            let (tile_rect, color) = match t {
                WandererTile::LeftHanded(rect, _) => {
                    (rect.pad(TILES_PADDING as f32), Color::Skobeloff)
                }
                WandererTile::RightHanded(rect, _) => (
                    rect.pad(TILES_PADDING as f32),
                    Color::InternationalOrangeGoldenGateBridge,
                ),
            };

            draw.background().color(Rgb::from(Color::ChampagnePink));

            draw.rect()
                .x_y(tile_rect.x(), tile_rect.y())
                .w_h(tile_rect.w(), tile_rect.h())
                .color(Rgb::from(color));
        }
    }

    draw.to_frame(app, &frame)
        .expect("There was a problem drawing the current frame.");
}
