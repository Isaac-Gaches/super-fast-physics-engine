use crate::maths::vec::Vec2;
use crate::render::{Colour};

pub struct Balls{
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub px: Vec<f32>,
    pub py: Vec<f32>,
    pub colour: Vec<Colour>,
}

impl Balls{
    pub fn new(max_balls: usize) -> Self {
        Self {
            x:      Vec::with_capacity(max_balls),
            y:      Vec::with_capacity(max_balls),
            px:     Vec::with_capacity(max_balls),
            py:     Vec::with_capacity(max_balls),
            colour: Vec::with_capacity(max_balls),
        }
    }

    pub fn add_ball(&mut self, pos: Vec2,vel:Vec2,colour: Colour) {
        self.x.push(pos.x);
        self.y.push(pos.y);
        self.px.push(pos.x-vel.x);
        self.py.push(pos.y-vel.y);
        self.colour.push(colour);
    }
}

pub struct UnsafeBalls(pub *mut Balls);

unsafe impl Send for UnsafeBalls {}
unsafe impl Sync for UnsafeBalls {}