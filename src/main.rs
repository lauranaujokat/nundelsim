use raylib::prelude::*;

const WIDTH: i32 = 2560;
const HEIGHT: i32 = 1080;
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
    fn gravity(&mut self, frame_time: f32) {
        dbg!(self.velocity);
        self.velocity += GRAVITY * frame_time;
    }
    fn apply_speed(&mut self, frame_time: f32) {
        self.position += self.velocity * frame_time;
    }
}

fn main() {
    let (mut raylib_handle, thread) = raylib::init().size(WIDTH, HEIGHT).title("nudelsim").build();
    raylib_handle.set_target_fps(60);
    let mut ball = Ball::new(
        Vector2::new((WIDTH / 2) as f32, 0.),
        Vector2::new(0., 0.),
        Color::BLACK,
        10.,
    );

    while !raylib_handle.window_should_close() {
        let frame_time = raylib_handle.get_frame_time();
        let mut draw_handle = raylib_handle.begin_drawing(&thread);
        ball.draw(&mut draw_handle);
        ball.gravity(frame_time);
        ball.apply_speed(frame_time);

        draw_handle.clear_background(Color::WHITE);
    }
}
