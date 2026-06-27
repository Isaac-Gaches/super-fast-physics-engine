use crate::simulation::balls::Balls;
use crate::simulation::grid::Grid;
use rayon::prelude::*;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub fn compute_cell_ids(
    balls: &Balls,
    grid_w: usize,
    cell_size: f32,
    particle_cell: &mut [u32],
) {
    let x = &balls.x;
    let y = &balls.y;

    let inv = 1.0 / cell_size;
    let gw = grid_w as u32;

    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            unsafe {
                compute_cell_ids_avx2(
                    x,
                    y,
                    inv,
                    gw,
                    particle_cell,
                );
                return;
            }
        }
    }

    particle_cell
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, cell)| {
            let cx = (x[i] * inv) as u32;
            let cy = (y[i] * inv) as u32;
            *cell = cx + cy * gw;
        });
}

#[target_feature(enable = "avx2")]
unsafe fn compute_cell_ids_avx2(
    x: &[f32],
    y: &[f32],
    inv: f32,
    gw: u32,
    out: &mut [u32],
) {
    let n = x.len();

    let inv_v = _mm256_set1_ps(inv);
    let gw_v = _mm256_set1_epi32(gw as i32);

    let chunks = n / 8;

    out.par_chunks_mut(8)
        .enumerate()
        .take(chunks)
        .for_each(|(chunk_idx, dst)| {
            unsafe {
                let i = chunk_idx * 8;

                let px = _mm256_loadu_ps(x.as_ptr().add(i));
                let py = _mm256_loadu_ps(y.as_ptr().add(i));

                let cx = _mm256_cvttps_epi32(_mm256_mul_ps(px, inv_v));
                let cy = _mm256_cvttps_epi32(_mm256_mul_ps(py, inv_v));

                let row = _mm256_mullo_epi32(cy, gw_v);

                let cell = _mm256_add_epi32(cx, row);

                _mm256_storeu_si256(dst.as_mut_ptr() as *mut __m256i, cell, );
            }
        });

    for i in (chunks * 8)..n {
        let cx = (x[i] * inv) as u32;
        let cy = (y[i] * inv) as u32;
        out[i] = cx + cy * gw;
    }
}

pub fn build_histogram(particle_cell: &[u32], cell_count: &mut [u8]) {
    cell_count.fill(0);

    let cell_len = cell_count.len();
    const CHUNK_SIZE: usize = 8192;

    let partials: Vec<Vec<u8>> = particle_cell
        .par_chunks(CHUNK_SIZE)
        .map_init(
            || vec![0; cell_len],
            |hist, chunk| {
                hist.fill(0);

                for &cell in chunk {
                    hist[cell as usize] += 1;
                }

                hist.clone()
            },
        )
        .collect();

    for hist in partials {
        for (dst, src) in cell_count.iter_mut().zip(hist) {
            *dst += src;
        }
    }
}

pub fn build_prefix_sum(cell_count: &[u8], cell_start: &mut [u32]) {
    let mut sum = 0;
    for (dst, &count) in cell_start.iter_mut().zip(cell_count) {
        *dst = sum;
        sum += count as u32;
    }
}

pub fn scatter_particles(
    particle_cell: &[u32],
    particle_ids: &mut [u32],
    cell_start: &[u32],
    cursor: &mut Vec<u32>,
) {
    cursor.copy_from_slice(cell_start);

    for i in 0..particle_cell.len() {
        let cell = particle_cell[i] as usize;
        let dst = cursor[cell] as usize;
        cursor[cell] += 1;
        particle_ids[dst] = i as u32;
    }
}

pub fn build_grid(balls: &Balls, grid: &mut Grid) {
    let n = balls.x.len();

    if grid.particle_cell.len() != n {
        grid.particle_cell.resize(n, 0);
        grid.particle_ids.resize(n, 0);
    }

    compute_cell_ids(
        balls,
        grid.grid_w,
        grid.cell_size,
        &mut grid.particle_cell,
    );

    build_histogram(
        &grid.particle_cell,
        &mut grid.cell_count,
    );

    build_prefix_sum(&grid.cell_count, &mut grid.cell_start);

    scatter_particles(
        &grid.particle_cell,
        &mut grid.particle_ids,
        &grid.cell_start,
        &mut grid.cursor,
    );
}