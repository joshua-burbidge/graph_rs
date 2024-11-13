use application::{femtovg_init, handler::MyApplicationHandler};
use winit::event_loop::EventLoop;

pub mod application;
pub mod grapher;

fn main() {
    let event_loop = EventLoop::new().expect("failed to create event loop");

    let (context, canvas, window, surface) = femtovg_init::init_canvas(&event_loop);
    let default_scale = 50.;

    let mut app = MyApplicationHandler::new(window, context, surface, canvas, default_scale);

    event_loop.run_app(&mut app).expect("run failed");
}
