use rand::prelude::*;
use raylib::prelude::*;

mod math;
use math::*;

const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 480;
const GRID_SIZE: usize = 40;
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

struct Cell<'a> {
    pub corners: &'a [Vector2; 4],
    pub abs_offset_x: usize,
    pub abs_offset_y: usize,
}

fn process_cell(cell: Cell, draw_handle: &mut RaylibDrawHandle) {
    let find_dots = |x: f32, y: f32| {
        [(x, y), (x - 1., y), (x, y - 1.), (x - 1., y - 1.)]
            .into_iter()
            .map(|(x, y)| Vector2::new(x, y))
            .zip(cell.corners)
            .map(|(offset, corner)| offset.dot(*corner))
            .collect::<Vec<_>>()
    };

    let samples = 3;
    let size: f32 = 1. / samples as f32;
    let sample_offsets = {
        let mut offsets: Vec<(f32, f32)> = Vec::with_capacity(samples * samples);
        for x in 0..samples {
            for y in 0..samples {
                offsets.push((x as f32 * size, y as f32 * size))
            }
        }
        offsets
    };

    for (x, y) in sample_offsets {
        let dots = find_dots(x, y);
        let rel_radius = bilerp(
            dots[0],
            dots[1],
            dots[2],
            dots[3],
            smootherstep(x),
            smootherstep(y),
        ) * size
            / 2.;

        if rel_radius > 1. {
            panic!("bad radius");
        }

        let abs_radius = rel_radius.abs() * GRID_SIZE as f32;
        let abs_x = cell.abs_offset_x as f32 + (x + size / 2.) * GRID_SIZE as f32;
        let abs_y = cell.abs_offset_y as f32 + (y + size / 2.) * GRID_SIZE as f32;
        let red = (((100. + (rel_radius.abs() / (size / 2.))).clamp(0., 1.)) * 255.) as u8;

        draw_handle.draw_circle(
            abs_x as i32,
            abs_y as i32,
            abs_radius,
            Color::new(255, 0, 0, red),
        );
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .title("Noise")
        .build();
    let mut grid: Grid = [[0.0; GRID_W]; GRID_H];
    let mut angle_offset: f64 = 0.0;

    set_grid(&mut grid, &|_| random_angle());

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        angle_offset += 0.01;

        for i in 0..GRID_H - 1 {
            for j in 0..GRID_W - 1 {
                let corners = [
                    unit(grid[i][j] + angle_offset),
                    unit(grid[i][j + 1] + angle_offset),
                    unit(grid[i + 1][j] + angle_offset),
                    unit(grid[i + 1][j + 1] + angle_offset),
                ];

                process_cell(
                    Cell {
                        corners: &corners,
                        abs_offset_x: j * GRID_SIZE,
                        abs_offset_y: i * GRID_SIZE,
                    },
                    &mut d,
                );
            }
        }
    }
}
