use nannou::{
    geom::{Rect, Vector2},
    noise::{NoiseFn, Seedable},
    prelude::*,
};

pub struct Noise<T>
where
    T: Seedable + NoiseFn<nannou::noise::Point3<f64>>,
{
    generator: T,
    z_offset: f64,
    xy_increment: f64,
    z_increment: f64,
}

impl<T> Noise<T>
where
    T: Seedable + NoiseFn<nannou::noise::Point3<f64>>,
{
    pub fn new(generator: T, z_offset: f64, xy_increment: f64, z_increment: f64) -> Self {
        Self {
            generator,
            z_offset,
            xy_increment,
            z_increment,
        }
    }
}

pub struct Flowfield<T>
where
    T: Seedable + NoiseFn<nannou::noise::Point3<f64>>,
{
    rows: u32,
    columns: u32,
    resolution: u32,
    pub canvas: Rect,
    vectors: Vec<Vector2>,
    noise: Noise<T>,
}

impl<T> Flowfield<T>
where
    T: Seedable + NoiseFn<nannou::noise::Point3<f64>>,
{
    pub fn new(canvas: Rect, noise: Noise<T>, resolution: u32) -> Self {
        let rows = (canvas.h() / resolution as f32).floor() as u32 + 1;
        let columns = (canvas.w() / resolution as f32).floor() as u32 + 1;

        let vectors = vec![Vector2::zero(); (rows * columns) as usize];

        Self {
            rows,
            columns,
            resolution,
            canvas,
            vectors,
            noise,
        }
    }

    pub fn update(&mut self) {
        let mut x_offset = 0.0;

        for row in 0..self.rows {
            let mut y_offset = 0.0;

            for column in 0..self.columns {
                self.vectors[(column + row * self.columns) as usize] = Vector2::from_angle(
                    (self
                        .noise
                        .generator
                        .get([x_offset, y_offset, self.noise.z_offset])
                        * TAU_F64) as f32,
                );
                y_offset += self.noise.xy_increment
            }

            x_offset += self.noise.xy_increment;
        }

        self.noise.z_offset += self.noise.z_increment;
    }

    fn display(&self, draw: &Draw) {
        for row in 0..self.rows {
            for column in 0..self.columns {
                let x = (self.canvas.left() + (self.resolution as f32 / 2.0))
                    + (self.resolution * column) as f32;
                let y = (self.canvas.top() - (self.resolution as f32 / 2.0))
                    - (self.resolution * row) as f32;

                draw.translate(vec3(x, y, 0.0))
                    .line()
                    .rotate(self.vectors[(column + row * self.columns) as usize].angle())
                    .color(rgb8(0, 0, 0))
                    .weight(1.0)
                    .points(pt2(0.0, 0.0), pt2(self.resolution as f32 / 2.0, 0.0));
            }
        }
    }

    pub fn get_vector_at(&self, point: &Point2) -> Option<&Vector2> {
        let mapped_x = map_range(
            point.x,
            self.canvas.left(),
            self.canvas.right(),
            0.0,
            self.canvas.w(),
        );
        let mapped_y = map_range(
            point.y,
            self.canvas.bottom(),
            self.canvas.top(),
            0.0,
            self.canvas.h(),
        );

        let x = (mapped_x / self.resolution as f32).floor();
        let y = (mapped_y / self.resolution as f32).floor();

        self.vectors.get((x + y * self.columns as f32) as usize)
    }
}
