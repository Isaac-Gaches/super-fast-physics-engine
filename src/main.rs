use winit::error::EventLoopError;
use winit::event_loop::EventLoop;
use crate::app::App;

mod simulation;
mod render;
mod app;
mod maths;

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new()?;
    let mut app = App::new();

    event_loop.run_app(&mut app)
}
