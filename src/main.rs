use raylib::prelude::*;
const WIDTH: i32 = 220;
const HEIGHT: i32 = 220;

fn main() {
    let (mut raylib_handle, thread) = raylib::init().size(WIDTH, HEIGHT).title("Hello, World").build();

    while !raylib_handle.window_should_close() {
        let mut draw_handle = raylib_handle.begin_drawing(&thread);

        draw_handle.clear_background(Color::WHITE);
        draw_handle.draw_circle(WIDTH/2, HEIGHT/2, 48.0, Color::BLACK);
        draw_handle.draw_fps(10,10);
    }
}
