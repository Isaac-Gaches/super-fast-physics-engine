use crate::maths::vec::Vec2;
use crate::simulation::balls::Balls;

use rayon::prelude::*;
use crate::simulation::grid::Grid;

pub fn integrate(balls: &mut Balls,grid: &Grid, dt: f32, gravity: Vec2) {
    let gdt2 = gravity * dt * dt;
    const CHUNK_SIZE: usize = 1024;
    let w = grid.grid_w as f32 * grid.cell_size;
    let h = grid.grid_h as f32 * grid.cell_size;

    balls
        .x
        .par_chunks_mut(CHUNK_SIZE)
        .zip(balls.y.par_chunks_mut(CHUNK_SIZE))
        .zip(balls.px.par_chunks_mut(CHUNK_SIZE))
        .zip(balls.py.par_chunks_mut(CHUNK_SIZE))
        .for_each(|(((x, y), px), py)| {
            for i in 0..x.len() {
                let vx = x[i] - px[i];
                let vy = y[i] - py[i];

                let nx = x[i] + vx + gdt2.x;
                let ny = y[i] + vy + gdt2.y;

                px[i] = x[i];
                py[i] = y[i];

                x[i] = nx.clamp(1.0, w - 1.0);
                y[i] = ny.clamp(1.0, h - 1.0);
            }
        });
}