use crate::simulation::balls::Balls;
use crate::simulation::grid::Grid;

use rayon::prelude::*;
use std::cell::UnsafeCell;

const STIFFNESS: f32 = 0.6;
const PUSH_SCALE: f32 = 0.5 * STIFFNESS;
const MAX_OVERLAP: f32 = 0.6;

const COLLISION_DIST_SQ: f32 = 4.0;
const MIN_DIST_SQ: f32 = 1e-10;

const STRIPE_WIDTH: usize = 16;

const NEIGHBOURS: [(i32, i32); 5] = [
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

struct Shared<T>(UnsafeCell<T>);

unsafe impl<T> Sync for Shared<T> {}

#[inline]
pub fn resolve_collisions(
    balls: &mut Balls,
    grid: &Grid,
) {
    let shared = Shared(UnsafeCell::new(balls));

    run_pass(&shared, grid, 0);
    run_pass(&shared, grid, 1);
}

#[inline]
fn run_pass(
    balls: &Shared<&mut Balls>,
    grid: &Grid,
    parity: usize,
) {
    let stripes = grid.grid_w.div_ceil(STRIPE_WIDTH);

    (parity..stripes)
        .step_by(2)
        .collect::<Vec<_>>()
        .into_par_iter()
        .for_each(|stripe| unsafe {
            let balls = &mut **balls.0.get();
            process_stripe(balls, grid, stripe);
        });
}

#[inline(always)]
unsafe fn process_stripe(
    balls: &mut Balls,
    grid: &Grid,
    stripe: usize,
) {
    let xs = balls.x.as_mut_ptr();
    let ys = balls.y.as_mut_ptr();

    let ids = &grid.particle_ids;
    let starts = &grid.cell_start;
    let counts = &grid.cell_count;

    let grid_w = grid.grid_w;
    let grid_h = grid.grid_h;

    let start_x = stripe * STRIPE_WIDTH;
    let end_x = ((stripe + 1) * STRIPE_WIDTH).min(grid_w);

    for cy in 0..grid_h {
        for cx in start_x..end_x {
            let cell = cx + cy * grid_w;

            let a_start = *starts.get_unchecked(cell) as usize;
            let a_count = *counts.get_unchecked(cell) as usize;

            if a_count == 0 {
                continue;
            }

            for &(ox, oy) in &NEIGHBOURS {
                let nx = cx as i32 + ox;
                let ny = cy as i32 + oy;

                if nx < 0 || ny < 0 || nx >= grid_w as i32 || ny >= grid_h as i32 {
                    continue;
                }

                let other = nx as usize + ny as usize * grid_w;

                let b_start = *starts.get_unchecked(other) as usize;
                let b_count = *counts.get_unchecked(other) as usize;

                if b_count == 0 {
                    continue;
                }

                collide_cells(
                    xs,
                    ys,
                    ids,
                    cell,
                    other,
                    a_start,
                    a_count,
                    b_start,
                    b_count,
                );
            }
        }
    }
}

#[inline(always)]
unsafe fn collide_cells(
    xs: *mut f32,
    ys: *mut f32,
    ids: &[u32],
    a_cell: usize,
    b_cell: usize,
    a_start: usize,
    a_count: usize,
    b_start: usize,
    b_count: usize,
) {
    for ai in 0..a_count {
        let a = *ids.get_unchecked(a_start + ai) as usize;

        let mut ax = *xs.add(a);
        let mut ay = *ys.add(a);

        let start = if a_cell == b_cell {
            ai + 1
        } else {
            0
        };

        for bi in start..b_count {
            let b = *ids.get_unchecked(b_start + bi) as usize;

            let bx = *xs.add(b);
            let by = *ys.add(b);

            let dx = bx - ax;
            let dy = by - ay;

            let dist_sq = dx * dx + dy * dy;

            if dist_sq <= MIN_DIST_SQ || dist_sq >= COLLISION_DIST_SQ {
                continue;
            }

            let dist = dist_sq.sqrt();
            let inv_dist = dist.recip();

            let overlap = (2.0 - dist).min(MAX_OVERLAP);
            let push = overlap * PUSH_SCALE * inv_dist;

            let px = dx * push;
            let py = dy * push;

            ax -= px;
            ay -= py;

            *xs.add(b) = bx + px;
            *ys.add(b) = by + py;
        }

        *xs.add(a) = ax;
        *ys.add(a) = ay;
    }
}