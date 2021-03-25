use nannou::geom::Rect;
use nannou::math::cgmath::MetricSpace;
use nannou::{
    event::Update,
    geom::{Range, Vector2},
    prelude::{BLACK, BLUE, GREEN, PINK, RED},
    App, Frame,
};
use ndarray::{s, Array, Ix2};
use rand::{self, distributions::uniform::SampleUniform, random, Rng};
use std::cmp::{max, min};
use std::f32::consts::TAU;
use std::ops::Add;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 640;
const N: u8 = 2;

struct Model {
    window_rect: Rect,
    r: f32,
    k: u8,
    cell_size: u32,
    background_grid: Array<Option<Vector2>, Ix2>,
    active_list: Vec<Vector2>,
}

impl Model {
    fn new(
        window_rect: Rect,
        r: f32,
        k: u8,
        cell_size: u32,
        background_grid: Array<Option<Vector2>, Ix2>,
        active_list: Vec<Vector2>,
    ) -> Self {
        Self {
            window_rect,
            r,
            k,
            cell_size,
            background_grid,
            active_list,
        }
    }
}

fn pick_random_point<S>(x: Range<S>, y: Range<S>) -> Vector2<S>
where
    S: PartialOrd + SampleUniform,
{
    let mut rng = rand::thread_rng();

    Vector2::from((
        rng.gen_range(x.start..=x.end),
        rng.gen_range(y.start..=y.end),
    ))
}

fn pick_random_index<T>(list: &Vec<T>) -> usize {
    (random::<f32>() * list.len() as f32).floor() as usize
}

fn pick_random_vector(magnitude_range: Range<f32>) -> Vector2<f32> {
    let mut rng = rand::thread_rng();

    let magnitude = rng.gen_range(magnitude_range.start..magnitude_range.end);

    Vector2::from_angle(rng.gen_range(0.0..TAU)).with_magnitude(magnitude)
}

fn convert_coordinate(coordinate: f32, from: Range<f32>, to: Range<f32>) -> f32 {
    to.start + (coordinate - from.start) * (to.end - to.start) / (from.end - from.start)
}

fn calculate_grid_indices(window_rect: &Rect, point: &Vector2, cell_size: u32) -> (usize, usize) {
    let cx = convert_coordinate(
        point.x,
        window_rect.x,
        Range::new(0.0, window_rect.w() - 1.0),
    );
    let cy = convert_coordinate(
        point.y,
        window_rect.y.invert(),
        Range::new(0.0, window_rect.h() - 1.0),
    );

    (
        (cx / cell_size as f32).floor() as usize,
        (cy / cell_size as f32).floor() as usize,
    )
}

fn insert_point_in_grid(
    point: Vector2,
    grid: &mut Array<Option<Vector2>, Ix2>,
    cell_size: u32,
    window_rect: &Rect,
) {
    let (x_index, y_index) = calculate_grid_indices(window_rect, &point, cell_size);

    println!("{}:{}", grid.shape()[0], grid.shape()[1]);

    grid.slice_mut(s![x_index, y_index]).fill(Some(point));
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Poisson-disc Sampling")
        .resizable(false)
        .view(view)
        .build()
        .expect("There was a problem creating the application's window.");

    let window_rect = match app.window(window_id) {
        None => panic!("Could not get the current window's rect."),
        Some(w) => w.rect(),
    };

    let r = 50.0;
    let k: u8 = 30;

    let cell_size = (r / (N as f32).sqrt()).floor() as u32;

    let grid_width = (WINDOW_WIDTH / cell_size) + 1;
    let grid_height = (WINDOW_HEIGHT / cell_size) + 1;

    let mut background_grid =
        Array::<Option<Vector2>, Ix2>::from_elem((grid_width as usize, grid_height as usize), None);

    let random_point = pick_random_point(window_rect.x, window_rect.y);

    insert_point_in_grid(random_point, &mut background_grid, cell_size, &window_rect);
    let active_list = vec![random_point];

    Model::new(window_rect, r, k, cell_size, background_grid, active_list)
}

fn is_point_valid(
    window_rect: &Rect,
    point: &Vector2,
    grid: &Array<Option<Vector2>, Ix2>,
    cell_size: u32,
    r: f32,
) -> bool {
    if point.x < window_rect.x.start
        || point.x > window_rect.x.end
        || point.y < window_rect.y.start
        || point.y > window_rect.y.end
    {
        return false;
    }

    let (x_index, y_index) = calculate_grid_indices(window_rect, &point, cell_size);

    let shape = grid.shape();

    let neighbourhood = grid.slice(s![
        max(x_index as i32 - 1, 0) as usize..=min(x_index as i32 + 1, shape[0] as i32 - 1) as usize,
        max(y_index as i32 - 1, 0) as usize..=min(y_index as i32 + 1, shape[1] as i32 - 1) as usize
    ]);

    neighbourhood.iter().all(|&p| match p {
        None => true,
        Some(p) => !(p.distance(*point) < r),
    })
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if !model.active_list.is_empty() {
        let index = pick_random_index(&model.active_list);

        let active_point = &model.active_list[index];

        let mut counter: u8 = 0;

        let new_point = loop {
            counter += 1;

            let v = pick_random_vector(Range::new(model.r, 2.0 * model.r));

            let new_point = active_point.add(v);

            if is_point_valid(
                &model.window_rect,
                &new_point,
                &model.background_grid,
                model.cell_size,
                model.r,
            ) {
                break Some(new_point);
            }

            if counter == model.k {
                break None;
            }
        };

        match new_point {
            None => {
                model.active_list.remove(index);
            }
            Some(p) => {
                insert_point_in_grid(
                    p,
                    &mut model.background_grid,
                    model.cell_size,
                    &model.window_rect,
                );
                model.active_list.push(p);
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for (i, r) in model.background_grid.outer_iter().enumerate() {
        for p in r.iter() {
            if let Some(p) = p {
                match i % 3 {
                    0 => {
                        draw.tri().w_h(40.0, 40.0).x_y(p.x, p.y).color(RED);
                    }
                    1 => {
                        draw.rect().w_h(40.0, 40.0).x_y(p.x, p.y).color(GREEN);
                    }
                    2 => {
                        draw.ellipse().w_h(40.0, 40.0).x_y(p.x, p.y).color(BLUE);
                    }
                    _ => {}
                }
            }
        }
    }

    //
    // for (i, p) in model.background_grid.iter().enumerate() {
    //     if let Some(p) = p {
    //         match i % 3 {
    //             0 => {
    //                 draw.tri().w_h(10.0, 20.0).x_y(p.x, p.y).color(RED);
    //             }
    //             1 => {
    //                 draw.rect().w_h(10.0, 10.0).x_y(p.x, p.y).color(GREEN);
    //             }
    //             2 => {
    //                 draw.ellipse().w_h(10.0, 10.0).x_y(p.x, p.y).color(BLUE);
    //             }
    //             _ => {}
    //         }
    //     }
    // }

    draw.to_frame(app, &frame)
        .expect("There was a problem drawing the current frame.");
}
