use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color};
use glutin::context::PossiblyCurrentContext;
use glutin::surface::Surface;
use glutin::{prelude::*, surface::WindowSurface};
use winit::application::ApplicationHandler;
use winit::event::{MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, NamedKey};
use winit::window::Window;
use winit::window::WindowId;

use super::femtovg_init;
use crate::grapher::equation::{Cubic, Linear, Quadratic};
use crate::grapher::graph::Graph;

#[derive(Default)]
pub struct MyApplicationHandler {
    close_requested: bool,
    scale: i32,
    window: Option<Window>,
    context: Option<PossiblyCurrentContext>,
    surface: Option<Surface<WindowSurface>>,
    canvas: Option<Canvas<OpenGl>>,
}

impl ApplicationHandler for MyApplicationHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let (context, mut canvas, window, surface) = femtovg_init::init_canvas(&event_loop);

        println!("resumed");
        canvas.set_size(1000, 600, window.scale_factor() as f32);

        self.window = Some(window);
        self.context = Some(context);
        self.surface = Some(surface);
        self.canvas = Some(canvas);
        self.scale = 50;
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.close_requested = true;
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let key = event.logical_key;

                match key {
                    Key::Named(NamedKey::Escape) => {
                        self.close_requested = true;
                    }
                    _ => (),
                }
            }
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(_x_delta, y_delta) => {
                    let new_scale = self.scale + y_delta as i32;

                    if new_scale != 0 {
                        self.scale = new_scale;
                        self.window.as_ref().unwrap().request_redraw();
                    }
                }
                _ => {}
            },
            WindowEvent::RedrawRequested => {
                render(
                    &self.context.as_ref().unwrap(),
                    &self.surface.as_ref().unwrap(),
                    &self.window.as_ref().unwrap(),
                    &mut self.canvas.as_mut().unwrap(),
                    self.scale,
                );
            }
            // _ => println!("{:?}", event),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.close_requested {
            event_loop.exit();
        }
    }
}

fn render_canvas(window: &Window, canvas: &mut Canvas<OpenGl>, scale: i32) {
    // Make sure the canvas has the right size:
    let size = window.inner_size();
    canvas.set_size(size.width, size.height, window.scale_factor() as f32);

    // clear canvas by filling with black
    canvas.clear_rect(0, 0, size.width, size.height, Color::black());

    let mut graph1 = Graph::new(size, scale, canvas);
    graph1.init_graph();

    // let eq1 = Linear { a: 0.5, b: -1. };
    // graph1.graph_linear(eq1);

    // let eq2 = Quadratic {
    //     a: 0.5,
    //     b: 0.,
    //     c: -1.,
    // };
    // graph1.graph_poly(eq2);

    let eq3 = Cubic {
        a: 0.2,
        b: -2.,
        c: 1.,
        d: 0.,
    };
    graph1.graph_poly(eq3);
}

fn render(
    context: &PossiblyCurrentContext,
    surface: &Surface<WindowSurface>,
    window: &Window,
    canvas: &mut Canvas<OpenGl>,
    scale: i32,
) {
    render_canvas(window, canvas, scale);

    // Tell renderer to execute all drawing commands
    canvas.flush();

    // Display what we've just rendered
    surface
        .swap_buffers(context)
        .expect("Could not swap buffers");
}
