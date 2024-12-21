use raylib::prelude::*;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;
const WANTED_FPS: i32 = 60;
const GRAVITY: Vector2 = Vector2::new(0., 9.8);

#[derive(Debug)]
struct Ball {
    position: Vector2,
    velocity: Vector2,
    color: Color,
    radius: f32,
}
impl Ball {
    fn new(position: Vector2, velocity: Vector2, color: Color, radius: f32) -> Self {
        Ball {
            position,
            velocity,
            color,
            radius,
        }
    }
    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_circle(
            self.position.x as i32,
            self.position.y as i32,
            self.radius,
            self.color,
        )
    }
    fn gravity(&mut self, dt: f32) {
        dbg!(self.velocity);
        self.velocity += GRAVITY * dt;
    }
    fn apply_speed(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }
}

fn main() {
    let (mut raylib_handle, thread) = raylib::init().size(WIDTH, HEIGHT).title("nudelsim").build();
    raylib_handle.set_target_fps(WANTED_FPS as u32);

    // 1 second per second
    let simulation_speed: f32 = 1.0;

    let mut ball = Ball::new(
        Vector2::new((WIDTH / 2) as f32, 0.),
        Vector2::new(0., 0.),
        Color::RED,
        10.,
    );

    while !raylib_handle.window_should_close() {
        let frame_time = raylib_handle.get_frame_time();
        let dt = frame_time / (1.0 / simulation_speed);

        let mut draw_handle = raylib_handle.begin_drawing(&thread);
        ball.draw(&mut draw_handle);
        ball.gravity(dt);
        ball.apply_speed(dt);

        draw_handle.clear_background(Color::BLACK);
    }
}
