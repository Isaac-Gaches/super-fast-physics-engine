use crate::maths::vec::Vec2;
use crate::simulation::balls::Balls;
use crate::simulation::grid::Grid;

use rayon::prelude::*;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

const CHUNK_SIZE: usize = 8192;

pub fn integrate(
    balls: &mut Balls,
    grid: &Grid,
    dt: f32,
    gravity: Vec2,
) {
    let gx = gravity.x * dt * dt;
    let gy = gravity.y * dt * dt;

    let w = grid.grid_w as f32 * grid.cell_size - 1.0;
    let h = grid.grid_h as f32 * grid.cell_size - 1.0;

    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            unsafe {
                integrate_avx2(balls, gx, gy, w, h);
                return;
            }
        }
    }

    integrate_scalar(balls, gx, gy, w, h);
}

#[inline(always)]
fn integrate_scalar(
    balls: &mut Balls,
    gx: f32,
    gy: f32,
    w: f32,
    h: f32,
) {
    balls
        .x
        .par_chunks_mut(CHUNK_SIZE)
        .zip(balls.y.par_chunks_mut(CHUNK_SIZE))
        .zip(balls.px.par_chunks_mut(CHUNK_SIZE))
        .zip(balls.py.par_chunks_mut(CHUNK_SIZE))
        .for_each(|(((xs, ys), pxs), pys)| {
            for i in 0..xs.len() {
                let x = xs[i];
                let y = ys[i];

                let nx = x + (x - pxs[i]) + gx;
                let ny = y + (y - pys[i]) + gy;

                pxs[i] = x;
                pys[i] = y;

                xs[i] = nx.clamp(1.0, w);
                ys[i] = ny.clamp(1.0, h);
            }
        });
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn integrate_avx2(
    balls: &mut Balls,
    gx: f32,
    gy: f32,
    w: f32,
    h: f32,
) {
    let min = _mm256_set1_ps(1.0);
    let max_x = _mm256_set1_ps(w);
    let max_y = _mm256_set1_ps(h);

    let gx_v = _mm256_set1_ps(gx);
    let gy_v = _mm256_set1_ps(gy);

    balls
        .x
        .par_chunks_mut(CHUNK_SIZE)
        .zip(balls.y.par_chunks_mut(CHUNK_SIZE))
        .zip(balls.px.par_chunks_mut(CHUNK_SIZE))
        .zip(balls.py.par_chunks_mut(CHUNK_SIZE))
        .for_each(|(((xs, ys), pxs), pys)| unsafe {
            let simd = xs.len() & !7;

            for i in (0..simd).step_by(8) {
                let x = _mm256_loadu_ps(xs.as_ptr().add(i));
                let y = _mm256_loadu_ps(ys.as_ptr().add(i));

                let px = _mm256_loadu_ps(pxs.as_ptr().add(i));
                let py = _mm256_loadu_ps(pys.as_ptr().add(i));

                let nx = _mm256_add_ps(_mm256_sub_ps(_mm256_add_ps(x, x), px), gx_v);
                let ny =_mm256_add_ps(_mm256_sub_ps(_mm256_add_ps(y, y), py), gy_v);

                _mm256_storeu_ps(pxs.as_mut_ptr().add(i), x);
                _mm256_storeu_ps(pys.as_mut_ptr().add(i), y);

                _mm256_storeu_ps(xs.as_mut_ptr().add(i), _mm256_max_ps(min, _mm256_min_ps(nx, max_x)), );
                _mm256_storeu_ps(ys.as_mut_ptr().add(i), _mm256_max_ps(min, _mm256_min_ps(ny, max_y)), );
            }

            for i in simd..xs.len() {
                let x = xs[i];
                let y = ys[i];

                let nx = x + (x - pxs[i]) + gx;
                let ny = y + (y - pys[i]) + gy;

                pxs[i] = x;
                pys[i] = y;

                xs[i] = nx.clamp(1.0, w);
                ys[i] = ny.clamp(1.0, h);
            }
        });
}