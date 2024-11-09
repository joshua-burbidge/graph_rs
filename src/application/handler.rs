use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color};
use glutin::context::PossiblyCurrentContext;
use glutin::surface::Surface;
use glutin::{prelude::*, surface::WindowSurface};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, NamedKey};
use winit::window::Window;
use winit::window::WindowId;

use super::femtovg_init;
use crate::grapher::equation::{Linear, Polynomial, PolynomialBuilder, Term};
use crate::grapher::graph::Graph;

#[derive(Default)]
pub struct MyApplicationHandler {
    close_requested: bool,
    scale: i32,
    dragging: bool,
    previous_position: Option<PhysicalPosition<f32>>,
    offset: PhysicalPosition<f32>,
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
        self.dragging = false;
        self.offset = PhysicalPosition::new(0., 0.);
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

                    if new_scale > 0 {
                        self.scale = new_scale;
                        self.window.as_ref().unwrap().request_redraw();
                    }
                }
                _ => {}
            },
            WindowEvent::MouseInput { state, .. } => match state {
                ElementState::Pressed => {
                    self.dragging = true;
                }
                ElementState::Released => {
                    self.dragging = false;
                    self.previous_position = None;
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                if !self.dragging {
                    return;
                }

                let new_position = position.cast::<f32>();
                match self.previous_position {
                    None => {
                        self.previous_position = Some(new_position);
                    }
                    Some(previous_position) => {
                        let delta_x = new_position.x - previous_position.x;
                        let delta_y = new_position.y - previous_position.y;

                        self.offset =
                            PhysicalPosition::new(self.offset.x + delta_x, self.offset.y + delta_y);

                        self.previous_position = Some(new_position);
                        self.window.as_ref().unwrap().request_redraw();
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                render(
                    &self.context.as_ref().unwrap(),
                    &self.surface.as_ref().unwrap(),
                    &self.window.as_ref().unwrap(),
                    &mut self.canvas.as_mut().unwrap(),
                    self.scale,
                    self.offset,
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

fn render_canvas(
    window: &Window,
    canvas: &mut Canvas<OpenGl>,
    scale: i32,
    offset: PhysicalPosition<f32>,
) {
    // Make sure the canvas has the right size:
    let size = window.inner_size();
    canvas.set_size(size.width, size.height, window.scale_factor() as f32);

    // clear canvas by filling with black
    canvas.clear_rect(0, 0, size.width, size.height, Color::black());

    let mut graph1 = Graph::new(size, scale, offset, canvas);
    graph1.init_graph();

    // let eq1 = Linear { a: 0.5, b: -1. };
    // graph1.graph_linear(eq1);

    let quad: Polynomial = PolynomialBuilder::new()
        .plus_x_squared_times(0.5)
        .plus_x_times(0.)
        .plus_const(-1.)
        .build();
    graph1.graph_poly(quad);

    let cubic: Polynomial = PolynomialBuilder::new()
        .plus_x_cubed_times(0.01)
        .plus_x_squared_times(-0.2)
        .plus_x_times(1.)
        .plus_const(0.)
        .build();
    graph1.graph_poly(cubic);

    let poly: Polynomial = PolynomialBuilder::new()
        .add_term(Term::plus_x_to_the(6).times(0.5))
        .plus_x_4th_times(-4.)
        .plus_x_squared_times(3.)
        .plus_const(-1.)
        .build();
    graph1.graph_poly(poly);
}

fn render(
    context: &PossiblyCurrentContext,
    surface: &Surface<WindowSurface>,
    window: &Window,
    canvas: &mut Canvas<OpenGl>,
    scale: i32,
    offset: PhysicalPosition<f32>,
) {
    render_canvas(window, canvas, scale, offset);

    // Tell renderer to execute all drawing commands
    canvas.flush();

    // Display what we've just rendered
    surface
        .swap_buffers(context)
        .expect("Could not swap buffers");
}
