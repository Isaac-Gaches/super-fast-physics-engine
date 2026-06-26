use bytemuck::{Pod, Zeroable};
use easy_gpu::assets::{Buffer, BufferUsages};
use easy_gpu::assets_manager::Handle;
use crate::maths::vec::Vec2;

pub struct Camera{
    pub buffer: Handle<Buffer>,
    view: View
}

impl Camera{
    pub fn new(egpu: &mut easy_gpu::Renderer)->Self{
        let view = View::new(egpu.window_aspect());

        let buffer = egpu.create_buffer_with_contents(
            BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            bytemuck::cast_slice(&[view])
        );

        Self{
            buffer,
            view,
        }
    }

    pub fn update(&mut self,egpu: &mut easy_gpu::Renderer){
        self.view.aspect = egpu.window_aspect();
        egpu.write_buffer(self.buffer,self.view);
    }
}

#[repr(C)]
#[derive(Clone, Copy,Pod,Zeroable)]
struct View{
    position: Vec2,
    zoom: f32,
    aspect: f32,
}

impl View{
    fn new(aspect:f32) -> Self{
        Self{
            position: Vec2::new(272., 256.0),
            zoom: 0.0018,
            aspect,
        }
    }
}