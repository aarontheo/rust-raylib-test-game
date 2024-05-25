use rand::rngs::ThreadRng;
use raylib::prelude::*;
use raylib::consts::MouseButton;
use std::{cmp, collections::{btree_map::Range, VecDeque}, f32};
use rand::{thread_rng, Rng};
use std::ptr::eq;

//STRUCTS
struct Body {
    pos: Vector2,
    vel: Vector2,
    size: f32,
    color: Color,
    mass: f32,
}

impl Body {

    fn draw(&self, d:&mut RaylibDrawHandle) {
        d.draw_circle_v(self.pos, self.size, self.color);
    }

    fn update(&mut self, rl:&mut RaylibHandle) {
        const BOUNCE_COEFF:f32 = 1.0;
        self.pos += self.vel;
        // let (mut xv, mut yv) = (self.vel.x, self.vel.y);
        // xv = xv.clamp(-MAX_SPEED, MAX_SPEED);
        // yv = yv.clamp(-MAX_SPEED, MAX_SPEED);
        // self.vel = Vector2::new(xv, yv);

        // SCREEN BOUNCE
        let (w, h) = (
            rl.get_screen_width(),
            rl.get_screen_height()
        );

        if self.pos.x-self.size < 0. || self.pos.x+self.size > w as f32 {
            self.vel.x *= -BOUNCE_COEFF;
        }
        if self.pos.y-self.size < 0. || self.pos.y+self.size > h as f32 {
            self.vel.y *= -BOUNCE_COEFF;
        }
    }

    fn apply_dir_force(&mut self, dir_in_rad:f32, force:f32) {
        //decompose radian direction into x and y components
        let (mut xf, mut yf) = dir_in_rad.sin_cos();
        //get x and y force components by multiplying each by force
        xf *= force;
        yf *= force;
        //assemble a force vector
        let force_vec = Vector2::new(xf, yf);
        //change the Body's velocity by the force values
        self.vel += force_vec;
    }
}

//MISC FUNCTIONS
fn calc_dist(point_a: Vector2, point_b: Vector2) -> f32 {
    // D = sqrt(a^2 + b^2)
    let (a, b) = (
        point_a.x - point_b.x,
        point_a.y - point_b.y
    );
    (a.powi(2) + b.powi(2)).sqrt()
}

fn calc_angle(point_a:Vector2, point_b:Vector2) -> f32 {
    let (a,b) = (
        point_b.x - point_a.x,
        point_b.y - point_a.y
    );

    a.atan2(b)
}

fn calc_grav(body: &Body, other_body: &Body) -> f32 {
    // F = (G * m1 * m2) / d^2
    const G:f32 = 0.5; //Adjust this as desired
    let r = body.size + other_body.size; //Radius to ignore gravity to prevent pinging around
    (G * body.mass * other_body.mass) / calc_dist(body.pos, other_body.pos).exclusive_clamp(-r, r).powi(2)
}

trait EX_CLAMP {
    fn exclusive_clamp(&self, min:f32, max:f32) -> f32;
}

impl EX_CLAMP for f32 {
    fn exclusive_clamp(&self, min:f32, max:f32) -> f32 {
        if *self > min && *self < max {
            if *self < max {
                max
            } else {
                min
            }
        } else {
            *self
        }
    }
}

// fn calc_radian_comps(radians: f32) -> (f32, f32) {
//     //decomposes a radian value into it's x and y counterparts.
//     //sin is y, cos is x
//     radians.sin_cos() //how convenient! I'll use this instead.
// }

// UPDATE, DRAW, AND MAIN FUNCTIONS
fn update(rl: &mut RaylibHandle, bodies: &mut Vec<Body>, rng: &mut ThreadRng) {
    // COMMENT THIS DECLARATION AND LOOP TO DISABLE GRAVITY
    //Calculate all net forces
    let mut body_forces:VecDeque<Vector2> = VecDeque::new();

    for body in bodies.iter() {
        let mut net_force = Vector2::new(0., 0.);
        for other_body in bodies.iter() {
            if !eq(body, other_body) { //don't apply a gravity force to a body to itself.
                //calculate the gravitational force between the two bodies
                let angle = calc_angle(body.pos, other_body.pos);
                let mag = calc_grav(body, other_body);
                
                //decompose angle into x and y
                let (mut xf, mut yf) = angle.sin_cos();
                
                // adjust each component by magnitude of g force
                xf *= mag;
                yf *= mag;

                // Finally, assemble into Vector2 and sum the force to the current net force of the body.
                net_force += Vector2::new(xf, yf);
            }
        }
        body_forces.push_back(net_force);
    }

    //Apply all net forces
    for body in bodies.iter_mut() {
        // COMMENTING THE MATCH BELOW DISABLES GRAVITY
        match body_forces.pop_front() {
            Some(force) => {
                body.vel += force
            },
            None => {
                println!("Nothing was in the Deque.");
            }
        }
        body.update(rl);
    }

    if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        const VEL_RANGE: std::ops::Range<f32> = -0.08..0.08;
        const MASS_RANGE: std::ops::Range<f32> = 0.2..1.;
        let mass: f32 = rng.gen_range(MASS_RANGE);
        let size:f32 = mass*25.;

        let Vector2{x, y} = rl.get_mouse_position();
        bodies.push(Body {
            pos: Vector2::new(x, y),
            vel: Vector2::new(rng.gen_range(VEL_RANGE), rng.gen_range(VEL_RANGE)),
            // vel: Vector2::new(0., 0.),
            size: size,
            color: Color::new(rng.gen(), rng.gen(), rng.gen(), 255/2),
            mass: mass,
        });
    }

    rl.enable_cursor();
}

fn draw(rl: &mut RaylibHandle, thread: &RaylibThread, bodies: &mut Vec<Body>) {
    let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);

    // Draw logic goes here
    d.clear_background(Color::WHITE);
    for body in bodies.iter() {
        body.draw(&mut d);
    }
}

// MAIN FUNCTION, GAME LOOP
fn main() {
    //Initialize stuff
    let (mut rl, thread) = init().size(640, 480).title("Hello, World").build();
    let mut bodies: Vec<Body> = Vec::new();

    let mut rng = thread_rng();

    while !&rl.window_should_close() {
        update(&mut rl, &mut bodies, &mut rng);

        // let mut d = rl.begin_drawing(&thread);
        // d.clear_background(Color::WHITE);
        // d.draw_circle_v(pos, 20., Color::new(100, 100, 50, 255));
        // pos += vel;

        draw(&mut rl, &thread, &mut bodies);
        // d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
