use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone,Debug,Pod,Zeroable)]
pub struct Colour{
    pub r: f32,
    pub g: f32,
    pub b: f32,
    _pad: f32,
}

impl Colour {
    pub fn new(r:f32,g:f32,b:f32)->Self{
        Self{
            r,
            g,
            b,
            _pad: 0.0,
        }
    }
}