use crate::math::*;
use raylib::prelude::Vector2;

#[derive(Debug)]
pub struct PerlinConfig {
    pub cell_size: usize,
    pub grid_width: usize,
    pub grid_height: usize,
}

#[derive(Debug)]
pub struct Perlin {
    pub grid: Vec<Vec<Vector2>>,
    pub config: PerlinConfig,
}

/// A vector space relative to a single cell [0, 1]
#[derive(Debug)]
struct Cell<'a> {
    pub corners: [&'a Vector2; 4],
    pub abs_offset_x: usize,
    pub abs_offset_y: usize,
    pub size: usize,
}

trait Interp {
    fn interpolate(&self, tl: f32, tr: f32, bl: f32, br: f32, x: f32, y: f32) -> f32;
}

#[allow(unused)]
struct SmoothStep;

impl Interp for SmoothStep {
    fn interpolate(&self, tl: f32, tr: f32, bl: f32, br: f32, x: f32, y: f32) -> f32 {
        bilerp(tl, tr, bl, br, smoothstep(x), smoothstep(y))
    }
}

struct SmootherStep;
impl Interp for SmootherStep {
    fn interpolate(&self, tl: f32, tr: f32, bl: f32, br: f32, x: f32, y: f32) -> f32 {
        bilerp(tl, tr, bl, br, smootherstep(x), smootherstep(y))
    }
}

#[derive(Debug, Clone)]
struct Absolute(f32, f32);
#[derive(Debug, Clone)]
struct Relative(f32, f32);

impl Cell<'_> {
    /// find dot products of a point within the cell
    pub fn dots(&self, pt: &Relative) -> Vec<f32> {
        [
            (pt.0, pt.1),
            (pt.0 - 1., pt.1),
            (pt.0, pt.1 - 1.),
            (pt.0 - 1., pt.1 - 1.),
        ]
        .into_iter()
        .map(|(x, y)| Vector2::new(x, y))
        .zip(self.corners)
        .map(|(offset, corner)| offset.dot(*corner))
        .collect()
    }

    fn abs_to_rel(&self, pt: Absolute) -> Relative {
        Relative(
            (pt.0 - self.abs_offset_x as f32) / self.size as f32,
            (pt.1 - self.abs_offset_y as f32) / self.size as f32,
        )
    }

    pub fn sample(&self, pt: Absolute, sampler: &dyn Interp) -> f32 {
        let rel = self.abs_to_rel(pt.clone());
        let dots = self.dots(&rel);

        sampler.interpolate(dots[0], dots[1], dots[2], dots[3], rel.0, rel.1)
    }
}

impl Perlin {
    fn cell<'a>(&'a self, pt: Absolute) -> Cell<'a> {
        if pt.0 >= 0.
            && pt.0 < self.config.grid_width as f32
            && pt.1 >= 0.
            && pt.1 < self.config.grid_height as f32
        {
            let abs_x = pt.0.floor();
            let abs_y = pt.1.floor();
            let x = abs_x as usize / self.config.cell_size as usize;
            let y = abs_y as usize / self.config.cell_size as usize;
            let corners = [
                &self.grid[y][x],
                &self.grid[y][x + 1],
                &self.grid[y + 1][x],
                &self.grid[y + 1][x + 1],
            ];
            Cell {
                corners,
                abs_offset_x: (x * self.config.cell_size) as _,
                abs_offset_y: (y * self.config.cell_size) as _,
                size: self.config.cell_size as _,
            }
        } else {
            panic!("Point out of bounds")
        }
    }

    pub fn random(config: PerlinConfig) -> Self {
        let cells_x = (config.grid_width as f32 / config.cell_size as f32).ceil() as usize;
        let cells_y = (config.grid_height as f32 / config.cell_size as f32).ceil() as usize;
        let mut grid = vec![vec![Vector2::zero(); cells_x + 1]; cells_y + 1];

        for row in &mut grid {
            for v in row {
                *v = unit_vector2(random_angle())
            }
        }

        Self { config, grid }
    }

    pub fn sample(&self, x: f32, y: f32) -> f32 {
        let cell = self.cell(Absolute(x, y));

        cell.sample(Absolute(x, y), &SmootherStep)
    }

    pub fn update_grid(&mut self, f: &dyn Fn(&mut Vector2)) {
        for row in &mut self.grid {
            for v in row {
                f(v)
            }
        }
    }
}
