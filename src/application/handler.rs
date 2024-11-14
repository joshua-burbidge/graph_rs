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

use crate::grapher::equation::Polynomial;
use crate::grapher::graph::Graph;

pub struct MyApplicationHandler {
    close_requested: bool,
    scale: f32,
    dragging: bool,
    previous_position: Option<PhysicalPosition<f32>>,
    offset: PhysicalPosition<f32>,
    window: Window,
    context: PossiblyCurrentContext,
    surface: Surface<WindowSurface>,
    canvas: Canvas<OpenGl>,
    equations: Vec<Polynomial>,
}

impl MyApplicationHandler {
    pub fn new(
        window: Window,
        context: PossiblyCurrentContext,
        surface: Surface<WindowSurface>,
        canvas: Canvas<OpenGl>,
        scale: f32,
        equations: Vec<Polynomial>,
    ) -> Self {
        let def_position = PhysicalPosition::<f32>::default();
        MyApplicationHandler {
            window,
            context,
            surface,
            canvas,
            equations,
            offset: def_position,
            previous_position: None,
            dragging: false,
            scale,
            close_requested: false,
        }
    }
}

impl ApplicationHandler for MyApplicationHandler {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        // try to render only after this happens?
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
            // make it zoom to the center of the screen
            // each scale increment means one more pixel per unit away from 0,0
            // (scale_change * units from target to 0,0) = offset change
            // (scale_change * offset / current scale) = offset change
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(_x_delta, y_delta) => {
                    let scale_increment = y_delta * 0.2; // adjust zoom speed

                    // log and exp so that the zoom speed feels the same when large and small
                    let new_scale = (self.scale.ln() + scale_increment).exp();

                    if new_scale > 1. {
                        let scale_change = new_scale - self.scale;
                        let offset_change_x = scale_change * self.offset.x / self.scale;
                        let offset_change_y = scale_change * self.offset.y / self.scale;

                        self.offset = PhysicalPosition::new(
                            self.offset.x + offset_change_x,
                            self.offset.y + offset_change_y,
                        );
                        self.scale = new_scale;
                        self.window.request_redraw();
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
            // now: center stays same position
            // therefore the cursor moves towards the center when zooming in, away when zooming out
            // move the center away from the cursor on a line?
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
                        self.window.request_redraw();
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                render(
                    &self.context,
                    &self.surface,
                    &self.window,
                    &mut self.canvas,
                    self.scale,
                    self.offset,
                    &self.equations,
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
    scale: f32,
    offset: PhysicalPosition<f32>,
    equations: &Vec<Polynomial>,
) {
    // Make sure the canvas has the right size:
    let size = window.inner_size();
    canvas.set_size(size.width, size.height, window.scale_factor() as f32);

    // clear canvas by filling with black
    canvas.clear_rect(0, 0, size.width, size.height, Color::black());

    let mut graph1 = Graph::new(size, scale, offset, canvas);
    graph1.init_graph();

    for equation in equations {
        graph1.graph_poly(equation);
    }
}

fn render(
    context: &PossiblyCurrentContext,
    surface: &Surface<WindowSurface>,
    window: &Window,
    canvas: &mut Canvas<OpenGl>,
    scale: f32,
    offset: PhysicalPosition<f32>,
    equations: &Vec<Polynomial>,
) {
    render_canvas(window, canvas, scale, offset, equations);

    // Tell renderer to execute all drawing commands
    canvas.flush();

    // Display what we've just rendered
    surface
        .swap_buffers(context)
        .expect("Could not swap buffers");
}
