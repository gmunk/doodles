use nannou::{
    Draw,
    {
        color::Rgba8,
        geom::{Point2, Rect, Vector2},
    },
};

pub struct Particle {
    pub position: Point2,
    previous_position: Option<Point2>,
    velocity: Vector2,
    acceleration: Vector2,
    velocity_limit: f32,
    color: Rgba8,
}

impl Particle {
    pub fn new(
        position: Point2,
        previous_position: Option<Point2>,
        velocity: Vector2,
        acceleration: Vector2,
        velocity_limit: f32,
        color: Rgba8,
    ) -> Self {
        Self {
            position,
            previous_position,
            velocity,
            acceleration,
            velocity_limit,
            color,
        }
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.limit_magnitude(self.velocity_limit);
        self.previous_position = Some(self.position);
        self.position += self.velocity;
        self.acceleration *= 0.0;
    }

    pub fn display(&self, draw: &Draw) {
        if let Some(previous_position) = self.previous_position {
            draw.line()
                .color(self.color)
                .weight(1.0)
                .points(previous_position, self.position);
        }
    }

    pub fn wrap_around(&mut self, canvas: &Rect) {
        if self.position.x > canvas.right() {
            self.position.x = canvas.left();
            self.previous_position = Some(self.position);
        }

        if self.position.x < canvas.left() {
            self.position.x = canvas.right();
            self.previous_position = Some(self.position);
        }

        if self.position.y > canvas.top() {
            self.position.y = canvas.bottom();
            self.previous_position = Some(self.position);
        }

        if self.position.y < canvas.bottom() {
            self.position.y = canvas.top();
            self.previous_position = Some(self.position);
        }
    }

    pub fn apply_force(&mut self, force: &Vector2) {
        self.acceleration += *force;
    }
}
