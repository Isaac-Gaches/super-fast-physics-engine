use bytemuck::{Pod, Zeroable};
use easy_gpu::assets::{BufferLayout, GpuVertex};
use easy_gpu::wgpu::{VertexFormat, VertexStepMode};

#[repr(C)]
#[derive(Clone, Copy,Pod,Zeroable)]
pub(super) struct Vertex{
    position: [f32; 2],
}

impl Vertex{
    pub fn new(position: [f32; 2]) -> Self {
        Self{
            position
        }
    }
}

impl GpuVertex for Vertex{
    fn buffer_layout() -> BufferLayout {
        BufferLayout::new()
            .step_mode(VertexStepMode::Vertex)
            .stride(size_of::<Self>() as u64)
            .attribute(0,0,VertexFormat::Float32x2)
    }
}