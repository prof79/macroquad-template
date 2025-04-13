// main.rs
// macroquad Template

use macroquad::prelude::*;
use macroquad::input::{KeyCode, is_key_down};
use rectangle::CustomRectangle;

mod rectangle;

#[macroquad::main("macroquad Template")]
async fn main() {

    let delta_x = 1.0;
    let delta_y = 1.0;

    let mut point1: (f32, f32) = (40.0, 40.0);
    let mut point2: (f32, f32) = (100.0, 200.0);

    let mut elapsed_time_seconds= 5.0;

    loop {
        clear_background(BLUE);

        draw_rectangle(300.0, 250.0, 220.0, 100.0, GREEN);

        let rect = CustomRectangle::new(
            300.0,
            400.0,
            300.0,
            150.0,
            1.0,
            WHITE,
        );

        rect.draw();

        if elapsed_time_seconds >= 5.0 {
            println!("{rect:?}");
            println!("{} x {}", rect.width(), rect.height());

            elapsed_time_seconds = 0.0;
        }

        elapsed_time_seconds += get_frame_time();

        draw_line(
            point1.0,
            point1.1,
            point2.0,
            point2.1,
            15.0,
            WHITE,
        );

        draw_rectangle(100.0, 100.0, 200.0, 80.0, ORANGE);

        draw_circle(200.0, 400.0, 1.0, YELLOW);

        if is_key_down(KeyCode::Down) {
            point1.1 += delta_y;
            point2.1 += delta_y;
        }
        else if is_key_down(KeyCode::Up) {
            point1.1 -= delta_y;
            point2.1 -= delta_y;
        }
        else if is_key_down(KeyCode::Left) {
            point1.0 -= delta_x;
            point2.0 -= delta_x;
        }
        else if is_key_down(KeyCode::Right) {
            point1.0 += delta_x;
            point2.0 += delta_x;
        }

        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}
