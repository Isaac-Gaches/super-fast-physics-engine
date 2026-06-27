use std::time::Instant;
use crate::maths::vec::Vec2;
use crate::render::{Colour};
use crate::simulation::balls::Balls;
use crate::simulation::broad_phase::build_grid;
use crate::simulation::grid::Grid;
use crate::simulation::integrate::{integrate};
use crate::simulation::narrow_phase::resolve_collisions;

pub struct World{
    balls: Balls,
    grid: Grid,
    colour_timer: Instant,
}

impl World {
    const GRAVITY: Vec2 = Vec2{
        x: 0.,
        y: -150.0
    };
    const MAX_BALLS: usize = 100000;
    pub fn new() -> Self {
        Self{
            balls: Balls::new(Self::MAX_BALLS),
            grid: Grid::new(288,288,2.0,Self::MAX_BALLS),
            colour_timer: Instant::now(),
        }
    }

    pub fn step(&mut self){
        if self.balls.x.len() < Self::MAX_BALLS {
            let t = self.colour_timer.elapsed().as_secs_f32()/2.0;
            for i in 0..50{
                self.balls.add_ball(
                    Vec2::new(5.0 + i as f32 * 0.1,400.0 + i as f32 * 2.0),
                    Vec2::new(0.3,0.1),
                    Colour::new((t.sin()+1.0)/2.0,(t.cos()+1.)/2.0,1.0-(t.cos()+1.0)/2.0)
                );
            }
        }

        let dt = (1. / 60.) / 8.;

        for i in 0..8{
            integrate(&mut self.balls,&self.grid, dt, Self::GRAVITY);
            if i % 3 == 0{
                build_grid(&self.balls, &mut self.grid);
            }
            resolve_collisions(&mut self.balls, &self.grid);
        }
    }

    pub fn extract(&self) -> (&Vec<f32>,&Vec<f32>, &Vec<Colour>) {
        (&self.balls.x,&self.balls.y,&self.balls.colour)
    }
}

