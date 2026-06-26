use crate::simulation::balls::Balls;
use crate::simulation::grid::Grid;
use rayon::prelude::*;

pub fn compute_cell_ids(
    balls: &Balls,
    grid_w: usize,
    cell_size: f32,
    particle_cell: &mut [u32],
) {
    let inv = 1.0 / cell_size;
    let gw = grid_w as u32;

    let x = &balls.x;
    let y = &balls.y;

    particle_cell
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, cell)| {
            let cx = (x[i] * inv) as u32;
            let cy = (y[i] * inv) as u32;
            *cell = cx + cy * gw;
        });
}

pub fn build_histogram(particle_cell: &[u32], cell_count: &mut [u32]) {
    cell_count.fill(0);

    let cell_len = cell_count.len();
    const CHUNK_SIZE: usize = 8192;

    let partials: Vec<Vec<u32>> = particle_cell
        .par_chunks(CHUNK_SIZE)
        .map_init(
            || vec![0u32; cell_len],
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

pub fn build_prefix_sum(cell_count: &[u32], cell_start: &mut [u32]) {
    let mut sum = 0;
    for (dst, &count) in cell_start.iter_mut().zip(cell_count) {
        *dst = sum;
        sum += count;
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