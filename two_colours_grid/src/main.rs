//! Rudimentary nannou app that explores grid positioning, colours and the toolkit's fundamentals.
//!
//! It fills a grid with coloured squares. Every tile's colour is chosen
//! randomly between two options. All the values are hardcoded.
//! This means that the user is not able to change the
//! dimensions of the grid and the squares colours. The app attempts to set up an organized
//! template for further exploration of nannou's capabilities.
use doodles_lib::color::Color;
use nannou::prelude::*;
use rand::random;

const WINDOW_WIDTH: u32 = 600;
const WINDOW_HEIGHT: u32 = 600;
const SQUARE_SIDE: u32 = 50;

trait Display {
    fn display(&self, draw: &Draw);
}

/// Square which will be displayed on the screen.
/// It has a location and a colour, its side is a constant.
struct Square {
    x: f32,
    y: f32,
    color: Color,
}

impl Square {
    /// Constructs a new square.
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            color: if random() {
                Color::DarkLava
            } else {
                Color::MiddleGrey
            },
        }
    }
}

impl Display for Square {
    /// Draws a square to the screen. Uses Nannou's Rect type.
    fn display(&self, draw: &Draw) {
        draw.rect()
            .color(Rgb::from(self.color))
            .w_h(SQUARE_SIDE as f32, SQUARE_SIDE as f32)
            .x_y(self.x, self.y);
    }
}

/// The application's model, consists of a background color and a vector of squares.
struct Model {
    background_color: Color,
    squares: Vec<Square>,
}

impl Model {
    /// Constructs a new model.
    fn new(background_color: Color, squares: Vec<Square>) -> Self {
        Self {
            background_color,
            squares,
        }
    }
}

impl Display for Model {
    /// Draws the application, loops over the vector of squares and draws each of them.
    fn display(&self, draw: &Draw) {
        draw.background().color(Rgb::from(self.background_color));

        for s in &self.squares {
            s.display(draw);
        }
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

/// Creates the application's model. This is by far the most important function in this app.
/// It creates a window where the grid of colored squares will be displayed. The Rect object of said
/// window is used to determine the position of each square. It finally creates the model
/// by passing a vec of squares and a background color to the appropriate constructor function.
fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Colourful Grid")
        .resizable(false)
        .view(view)
        .build()
        .expect("There was a problem creating a new window.");

    let window_rect = match app.window(window_id) {
        None => panic!("Could not get the current window's rect."),
        Some(w) => w.rect(),
    };

    let squares_num = (WINDOW_WIDTH * WINDOW_HEIGHT) / (SQUARE_SIDE * SQUARE_SIDE);

    let mut squares: Vec<Square> = Vec::new();

    for i in 0..squares_num {
        for j in 0..squares_num {
            let x = window_rect.left() + (SQUARE_SIDE * i) as f32;
            let y = window_rect.top() - (SQUARE_SIDE * j) as f32;

            squares.push(Square::new(x, y))
        }
    }

    Model::new(Color::Alabaster, squares)
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    model.display(&draw);

    draw.to_frame(app, &frame)
        .expect("There was a problem drawing the current frame.");
}
