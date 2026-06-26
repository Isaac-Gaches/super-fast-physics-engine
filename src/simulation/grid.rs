pub struct Grid {
    pub cell_start:        Vec<u32>,
    pub cell_count:        Vec<u32>,
    pub particle_ids:      Vec<u32>,
    pub particle_cell:     Vec<u32>,
    pub grid_w:            usize,
    pub grid_h:            usize,
    pub cell_size:         f32,
    pub cursor:            Vec<u32>,
}

impl Grid {
    pub fn new(width: usize, height: usize, cell_size: f32, max_balls: usize) -> Self {
        let grid_size = width * height;
        Self {
            cell_start:        vec![0u32; grid_size],
            cell_count:        vec![0u32; grid_size],
            particle_ids:      Vec::with_capacity(max_balls),
            particle_cell:     Vec::with_capacity(max_balls),
            cursor:            vec![0u32; grid_size],
            grid_w:            width,
            grid_h:            height,
            cell_size,
        }
    }
}
