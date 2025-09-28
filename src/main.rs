use ::core::f32;

use raylib::prelude::*;

mod math;
mod perlin;

use perlin::*;

const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 480;
const CELL_SIZE: usize = 40;
const RADIANS_PER_SECOND: f32 = f32::consts::PI * 4.;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .title("Noise")
        .build();
    let config = PerlinConfig {
        cell_size: CELL_SIZE as _,
        grid_height: WINDOW_HEIGHT as _,
        grid_width: WINDOW_WIDTH as _,
    };
    let mut perlin = Perlin::random(config);
    let mut now = std::time::Instant::now();
    while !rl.window_should_close() {
        let elapsed = now.elapsed().as_micros() as f32 / 1e6_f32;
        let radians = RADIANS_PER_SECOND * elapsed;
        perlin.update_grid(&|vec| vec.rotate(radians));

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        let radius = 5;
        for y in 0..(WINDOW_HEIGHT / radius) {
            for x in 0..(WINDOW_WIDTH / radius) {
                let px = x * radius;
                let py = y * radius;
                let v = perlin.sample(px as _, py as _) * radius as f32;
                d.draw_circle(px as _, py as _, v, Color::RED);
            }
        }
        now = std::time::Instant::now();
    }
}
