use crate::maths::vec::Vec2;
use crate::simulation::balls::Balls;
use crate::simulation::grid::Grid;

use rayon::prelude::*;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub fn integrate(
    balls: &mut Balls,
    grid: &Grid,
    dt: f32,
    gravity: Vec2,
) {
    let gdt2 = gravity * dt * dt;

    let w = grid.grid_w as f32 * grid.cell_size;
    let h = grid.grid_h as f32 * grid.cell_size;

    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            unsafe {
                integrate_avx2(
                    balls,
                    gdt2.x,
                    gdt2.y,
                    w,
                    h,
                );
                return;
            }
        }
    }

    // scalar fallback
    integrate_scalar(balls, gdt2.x, gdt2.y, w, h);
}

fn integrate_scalar(
    balls: &mut Balls,
    gx: f32,
    gy: f32,
    w: f32,
    h: f32,
) {
    const CHUNK_SIZE: usize = 1024;

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

                let nx = x[i] + vx + gx;
                let ny = y[i] + vy + gy;

                px[i] = x[i];
                py[i] = y[i];

                x[i] = nx.clamp(1.0, w - 1.0);
                y[i] = ny.clamp(1.0, h - 1.0);
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
    const CHUNK_SIZE: usize = 1024;

    let min_v = _mm256_set1_ps(1.0);
    let max_x = _mm256_set1_ps(w - 1.0);
    let max_y = _mm256_set1_ps(h - 1.0);

    let gx_v = _mm256_set1_ps(gx);
    let gy_v = _mm256_set1_ps(gy);

    balls
        .x
        .par_chunks_mut(CHUNK_SIZE)
        .zip(balls.y.par_chunks_mut(CHUNK_SIZE))
        .zip(balls.px.par_chunks_mut(CHUNK_SIZE))
        .zip(balls.py.par_chunks_mut(CHUNK_SIZE))
        .for_each(|(((x, y), px), py)| unsafe {
            let simd_end = x.len() & !7;

            for i in (0..simd_end).step_by(8) {
                let x0 = _mm256_loadu_ps(x.as_ptr().add(i));
                let y0 = _mm256_loadu_ps(y.as_ptr().add(i));

                let px0 = _mm256_loadu_ps(px.as_ptr().add(i));
                let py0 = _mm256_loadu_ps(py.as_ptr().add(i));

                let vx = _mm256_sub_ps(x0, px0);
                let vy = _mm256_sub_ps(y0, py0);

                let nx = _mm256_add_ps(_mm256_add_ps(x0, vx), gx_v, );
                let ny = _mm256_add_ps(_mm256_add_ps(y0, vy), gy_v, );

                _mm256_storeu_ps(px.as_mut_ptr().add(i), x0, );
                _mm256_storeu_ps(py.as_mut_ptr().add(i), y0, );

                let nx = _mm256_max_ps(min_v, _mm256_min_ps(nx, max_x), );
                let ny = _mm256_max_ps(min_v, _mm256_min_ps(ny, max_y), );

                _mm256_storeu_ps(x.as_mut_ptr().add(i), nx, );
                _mm256_storeu_ps(y.as_mut_ptr().add(i), ny, );
            }

            for i in simd_end..x.len() {
                let vx = x[i] - px[i];
                let vy = y[i] - py[i];

                let nx = x[i] + vx + gx;
                let ny = y[i] + vy + gy;

                px[i] = x[i];
                py[i] = y[i];

                x[i] = nx.clamp(1.0, w - 1.0);
                y[i] = ny.clamp(1.0, h - 1.0);
            }
        });
}