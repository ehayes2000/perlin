use rand::prelude::*;
use raylib::prelude::*;

mod math;
use math::*;

const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 480;
const GRID_SIZE: usize = 10;
const GRID_W: usize = WINDOW_WIDTH / GRID_SIZE + 1;
const GRID_H: usize = WINDOW_HEIGHT / GRID_SIZE + 1;

type Grid = [[f64; GRID_W]; GRID_H];

fn set_grid(grid: &mut Grid, f: &dyn Fn(f64) -> f64) {
    for row in grid {
        for e in row {
            *e = f(*e);
        }
    }
}

fn random_angle() -> f64 {
    let mut rng = rand::rng();
    rng.random::<f64>() * 2.0 * std::f64::consts::PI
}

fn unit(angle: f64) -> Vector2 {
    Vector2::new(angle.cos() as f32, angle.sin() as f32)
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .title("Noise")
        .build();
    let mut grid: Grid = [[0.0; GRID_W]; GRID_H];
    let mut angle_offset: f64 = 0.0;

    let offset_vectors = [(-0.5, -0.5), (0.5, -0.5), (-0.5, 0.5), (0.5, 0.5)]
        .into_iter()
        .map(|(a, b)| Vector2::new(a, b))
        .collect::<Vec<_>>();
    set_grid(&mut grid, &|_| random_angle());

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        angle_offset += 0.015;
        for i in 0..GRID_H - 1 {
            for j in 0..GRID_W - 1 {
                let y = i * GRID_SIZE + GRID_SIZE / 2;
                let x = j * GRID_SIZE + GRID_SIZE / 2;

                let corners = [
                    grid[i][j] + angle_offset,
                    grid[i][j + 1] + angle_offset,
                    grid[i + 1][j] + angle_offset,
                    grid[i + 1][j + 1] + angle_offset,
                ]
                .into_iter()
                .map(unit)
                .collect::<Vec<_>>();

                let dots = offset_vectors
                    .iter()
                    .zip(corners.iter())
                    .map(|(offset, corner)| offset.dot(*corner))
                    .collect::<Vec<_>>();

                let interped = bilerp(
                    dots[0] as f64,
                    dots[1] as f64,
                    dots[2] as f64,
                    dots[3] as f64,
                    smootherstep(0.5),
                    smootherstep(0.5),
                ) * GRID_SIZE as f64;

                d.draw_circle(x as i32, y as i32, interped as f32, Color::RED);
            }
        }
    }
}
