use raylib::drawing::{RaylibDraw, RaylibDrawHandle};

enum ShapeType {
    Square(i32),
    Rect(i32, i32),
    Circle(f32),
}

fn draw(shape:ShapeType, color: raylib::color::Color, d:RaylibDrawHandle) {
    match shape {
        Square(l) => d.draw_rectangle(x, y, width, height, color),
    }
}

trait Drawable {
    fn get_shape(&self) -> ShapeType;
    fn draw(&self, d:RaylibDrawHandle) {
        self.get_shape().draw()
    }
}