use std::sync::Arc;
use easy_gpu::assets::{render_uniform, Buffer, BufferLayout, BufferUsages, GpuVertex, Material, MaterialBuilder, Mesh, RenderPipelineBuilder};
use easy_gpu::assets_manager::Handle;
use easy_gpu::wgpu::{TextureFormat, VertexFormat, VertexStepMode};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::render::{Colour};
use crate::render::camera::Camera;
use crate::render::vertex::Vertex;

pub struct Renderer {
    pub egpu: easy_gpu::Renderer,
    camera: Camera,
    ball_material: Handle<Material>,
    quad: Handle<Mesh>,
    ball_x_buffer: Handle<Buffer>,
    ball_y_buffer: Handle<Buffer>,
    ball_col_buffer: Handle<Buffer>,
    num_balls: u32,
}

impl Renderer {
    pub fn new(window: Arc<Window>) -> Self {
        let mut egpu = pollster::block_on(easy_gpu::Renderer::new(window))
            .clear_colour(0.,0.,0.,1.0);

        let shader = egpu.load_shader(include_str!("balls.wgsl"));

        let ball_x_buffer = egpu.create_buffer(
            BufferUsages::VERTEX | BufferUsages::COPY_DST,
            16777216
        );
        let ball_y_buffer = egpu.create_buffer(
            BufferUsages::VERTEX | BufferUsages::COPY_DST,
            16777216
        );
        let ball_col_buffer = egpu.create_buffer(
            BufferUsages::VERTEX | BufferUsages::COPY_DST,
            33554432
        );

        let camera = Camera::new(&mut egpu);

        let render_pipeline = RenderPipelineBuilder::new(shader)
            .vertex_layout(Vertex::buffer_layout())
            .vertex_layout(
                BufferLayout::new()
                    .stride(4)
                    .step_mode(VertexStepMode::Instance)
                    .attribute(1,0,VertexFormat::Float32)
            )
            .vertex_layout(
                BufferLayout::new()
                    .stride(4)
                    .step_mode(VertexStepMode::Instance)
                    .attribute(2,0,VertexFormat::Float32)
            )
            .vertex_layout(
                BufferLayout::new()
                    .stride(size_of::<[f32;4]>() as u64)
                    .step_mode(VertexStepMode::Instance)
                    .attribute(3,0,VertexFormat::Float32x3)
            )
            .material_layout(&[render_uniform(0)])
            .depth_format(TextureFormat::Depth24Plus)
            .depth_writes_enabled(false)
            .build(&mut egpu);

        let ball_material = MaterialBuilder::new(render_pipeline)
            .buffer(0,camera.buffer)
            .build(&mut egpu);

        let vertices = [
            Vertex::new([-1.0, -1.0]),
            Vertex::new([1.0, -1.0]),
            Vertex::new([1.0, 1.0]),
            Vertex::new([-1.0, 1.0])
        ];

        let indices = [0, 1, 2, 0, 2, 3];

        let quad = egpu.create_mesh(&vertices, &indices);

        Self{
            egpu,
            camera,
            ball_material,
            quad,

            ball_x_buffer,
            ball_y_buffer,
            ball_col_buffer,
            num_balls: 0,
        }
    }

    pub fn upload_balls(&mut self,balls: (&Vec<f32>,&Vec<f32>, &Vec<Colour>)) {
        self.egpu.write_array_buffer(self.ball_x_buffer,balls.0.as_slice());
        self.egpu.write_array_buffer(self.ball_y_buffer,balls.1.as_slice());
        self.egpu.write_array_buffer(self.ball_col_buffer,balls.2.as_slice());
        self.num_balls = balls.0.len() as u32;
    }

    pub fn draw(&mut self){
        let frame = self.egpu.begin_frame();

        frame.draw_manual_batch(
            vec![self.ball_x_buffer,self.ball_y_buffer, self.ball_col_buffer],
            self.ball_material,
            self.quad,
            0..self.num_balls
        );

        self.egpu.render();
    }
    
    pub fn resize(&mut self,size: PhysicalSize<u32>) {
        self.egpu.resize_surface(size);
        self.camera.update(&mut self.egpu);
    }
}