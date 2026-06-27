use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use crate::render::{Renderer};
use crate::simulation::World;

pub struct App{
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    world: World,
    frames: u32,
    timer: Instant,
}

impl App{
    pub fn new() -> Self {
        Self{
            window: None,
            renderer: None,
            world: World::new(),
            frames: 0,
            timer: Instant::now(),
        }
    }
}

impl ApplicationHandler for App{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop
            .create_window(Window::default_attributes())
            .expect("Failed to create window"));

        let renderer = Renderer::new(window.clone());

        self.window = Some(window);
        self.renderer = Some(renderer);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent)  {
        let renderer = self.renderer.as_mut().unwrap();

        match event {
            WindowEvent::KeyboardInput { event: _, .. } => {
            }
            WindowEvent::MouseInput { button: _,state: _,..} =>{

            }
            WindowEvent::CursorMoved {position: _,..} =>{

            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                renderer.resize(size);
            }
            WindowEvent::RedrawRequested => {
                self.frames += 1;
                if self.timer.elapsed().as_secs() >= 1 {
                    println!("fps {}", self.frames);
                    self.timer = Instant::now();
                    self.frames = 0;
                }

                self.world.step();

                renderer.upload_balls(self.world.extract());

                renderer.draw();
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}