use raylib::prelude::*;
// use uuid::Uuid;

//MODULES
mod entity;

// INIT, UPDATE, AND DRAW FUNCTIONS
fn init() {}

fn update(rl: &mut RaylibHandle) {
    rl.get_mouse_position();
    rl.enable_cursor();
}

fn draw(rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(&thread);

    // Draw logic goes here
    d.clear_background(Color::WHITE);
}

// MAIN FUNCTION, GAME LOOP
fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    while !&rl.window_should_close() {
        update(&mut rl);
        draw(&mut rl, &thread);
        // d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
