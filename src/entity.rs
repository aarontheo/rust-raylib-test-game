use raylib::core::{math::Vector2, color::Color};

impl Entity {
    fn new(x: f32, y: f32, shape: ShapeType, color: Color) -> Entity {
        Entity {
            shape,
            color,
            position: Vector2::new(x, y),
            velocity: Vector2::new(0., 0.),
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        let Vector2 { x, y } = self.position;
        self.shape.draw(d, x as i32, y as i32, self.color);
    }
}

pub struct Entity {
    // uuid: Uuid,
    shape: ShapeType,
    color: Color,
    position: Vector2,
    velocity: Vector2,
}

trait HasPosition {
    fn get_position(&self) -> Vector2;
    fn set_position(&mut self, position: Vector2);
    fn update_position(&mut self, velocity: Vector2) {
        let pos = self.get_position();
        self.set_position(pos + velocity);
    }
}

trait HasVelocity {
    fn get_velocity(&self) -> Vector2;
    fn set_velocity(&mut self, velocity: Vector2);
    fn update_velocity(&mut self, force: Vector2) {
        let vel = self.get_velocity();
        self.set_velocity(vel + force);
    }
}

impl HasPosition for Entity {
    fn get_position(&self) -> Vector2 {
        self.position
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }
}

impl HasVelocity for Entity {
    fn get_velocity(&self) -> Vector2 {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: Vector2) {
        self.velocity = velocity;
    }
}

enum ShapeType {
    Rect { width: i32, height: i32 },
    Square(i32),
    Circle(f32),
}

use raylib::core::drawing::*;

impl ShapeType {
    fn draw(&self, d: &mut RaylibDrawHandle, x: i32, y: i32, color: Color) {
        match self {
            ShapeType::Circle(radius) => d.draw_circle(x, y, *radius, color),
            ShapeType::Rect { width, height } => d.draw_rectangle(x, y, *width, *height, color),
            ShapeType::Square(side_length) => {
                d.draw_rectangle(x, y, *side_length, *side_length, color)
            }
        }
    }
}
