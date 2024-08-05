use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};
use crate::app::app::App;

mod app;


fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app)
}
