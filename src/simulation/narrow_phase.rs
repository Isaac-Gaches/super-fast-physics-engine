use crate::simulation::balls::Balls;
use crate::simulation::grid::Grid;

use rayon::prelude::*;
use std::cell::UnsafeCell;

const RELAXATION: f32 = 0.4;
const MAX_OVERLAP_FRACTION: f32 = 0.3;

const COLLISION_DIST_SQ: f32 = 4.0;
const MIN_DIST_SQ: f32 = 1e-10;

const STRIPE_WIDTH: usize = 8;

const NEIGHBOURS: [(i32, i32); 5] = [
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

struct Shared<T>(UnsafeCell<T>);

unsafe impl<T> Sync for Shared<T> {}

pub fn resolve_collisions(
    balls: &mut Balls,
    grid: &Grid,
) {
    let shared = Shared(UnsafeCell::new(balls));

    run_pass(&shared, grid, 0);
    run_pass(&shared, grid, 1);
}

fn run_pass(
    balls: &Shared<&mut Balls>,
    grid: &Grid,
    parity: usize,
) {
    let stripes =
        grid.grid_w.div_ceil(STRIPE_WIDTH);

    (0..stripes)
        .into_par_iter()
        .filter(|s| s % 2 == parity)
        .for_each(|stripe| unsafe {
            process_stripe(
                &mut **balls.0.get(),
                grid,
                stripe,
            );
        });
}

unsafe fn process_stripe(
    balls: &mut Balls,
    grid: &Grid,
    stripe: usize,
) {
    let start_x = stripe * STRIPE_WIDTH;

    let end_x = ((stripe + 1) * STRIPE_WIDTH).min(grid.grid_w);

    for cy in 0..grid.grid_h {
        for cx in start_x..end_x {

            let cell = cx + cy * grid.grid_w;

            for &(ox, oy) in &NEIGHBOURS {
                let nx = cx as i32 + ox;
                let ny = cy as i32 + oy;

                if nx < 0 || ny < 0 || nx >= grid.grid_w as i32 || ny >= grid.grid_h as i32 {
                    continue;
                }

                let other = nx as usize + ny as usize * grid.grid_w;

                process_pair(
                    balls,
                    grid,
                    cell,
                    other,
                );
            }
        }
    }
}

#[inline(always)]
unsafe fn process_pair(
    balls: &mut Balls,
    grid: &Grid,
    a_cell: usize,
    b_cell: usize,
) {
    let a_start = grid.cell_start[a_cell] as usize;
    let a_count = grid.cell_count[a_cell] as usize;

    let b_start = grid.cell_start[b_cell] as usize;
    let b_count = grid.cell_count[b_cell] as usize;

    if a_count == 0 || b_count == 0 {
        return;
    }

    for ai in 0..a_count {
        let a = grid.particle_ids[a_start + ai] as usize;

        let ax = *balls.x.get_unchecked(a);

        let ay = *balls.y.get_unchecked(a);

        let start =
            if a_cell == b_cell {
                ai + 1
            } else {
                0
            };

        for bi in start..b_count {
            let b = grid.particle_ids[b_start + bi] as usize;

            let dx = balls.x[b] - ax;
            let dy = balls.y[b] - ay;

            let dist_sq = dx * dx + dy * dy;

            if !(MIN_DIST_SQ..COLLISION_DIST_SQ).contains(&dist_sq) {
                continue;
            }

            let inv = dist_sq.sqrt().recip();
            let overlap = (2.0 - dist_sq * inv).min(2.0 * MAX_OVERLAP_FRACTION, );
            let push = overlap * 0.5 * inv * RELAXATION;

            let px = dx * push;
            let py = dy * push;

            *balls.x.get_unchecked_mut(a) -= px;
            *balls.y.get_unchecked_mut(a) -= py;

            *balls.x.get_unchecked_mut(b) += px;
            *balls.y.get_unchecked_mut(b) += py;
        }
    }
}