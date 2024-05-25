enum ShapeType {
    Square(i32),
    Rect(i32, i32),
    Circle(f32),
}

impl ShapeType {
    fn draw(&self, d:raylib::draw::RaylibDrawHandle) {
        match self {
            Square(l) => d.RaylibDrawHandle()
        }
    }
}

trait Drawable {
    fn get_shape(&self) -> ShapeType;
    fn draw(&self, d:RaylibDrawHandle) {
        self.get_shape().draw()
    }
}