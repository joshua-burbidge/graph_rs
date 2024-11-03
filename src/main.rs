use application::handler::MyApplicationHandler;
use winit::event_loop::EventLoop;

pub mod application;

fn main() {
    let event_loop = EventLoop::new().expect("failed to create event loop");

    let mut app = MyApplicationHandler::default();

    event_loop.run_app(&mut app).expect("run failed");
}
