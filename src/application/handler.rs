use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, Paint, Path, Renderer};
use glutin::context::PossiblyCurrentContext;
use glutin::surface::Surface;
use glutin::{prelude::*, surface::WindowSurface};
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, NamedKey};
use winit::window::Window;
use winit::window::WindowId;

use crate::application::femtovg_init;

#[derive(Default)]
pub struct MyApplicationHandler {
    close_requested: bool,
    mouse_position: PhysicalPosition<f64>,
    window: Option<Window>,
    context: Option<PossiblyCurrentContext>,
    surface: Option<Surface<WindowSurface>>,
    canvas: Option<Canvas<OpenGl>>,
}

impl ApplicationHandler for MyApplicationHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let (context, mut canvas, window, surface) = femtovg_init::init_canvas(&event_loop);

        canvas.set_size(1000, 600, window.scale_factor() as f32);

        self.window = Some(window);
        self.context = Some(context);
        self.surface = Some(surface);
        self.canvas = Some(canvas);
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
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = position;
                self.window.as_ref().unwrap().request_redraw();
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
            WindowEvent::RedrawRequested => {
                render(
                    &self.context.as_ref().unwrap(),
                    &self.surface.as_ref().unwrap(),
                    &self.window.as_ref().unwrap(),
                    &mut self.canvas.as_mut().unwrap(),
                    self.mouse_position,
                );
            }
            // _ => println!("{:?} {:?}", window_id, event),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.close_requested {
            event_loop.exit();
        }
    }
}

// struct TickMark {
//     size: i32,
// }
struct Graph {
    size: PhysicalSize<u32>,
    scale: i32,
}

fn tick_marks(size: PhysicalSize<u32>, scale: i32) -> (Path, Path) {
    let midpoint_x = (size.width / 2) as i32;
    let midpoint_y = (size.height / 2) as i32;

    let num_x_ticks = midpoint_x / scale;
    let num_y_ticks = midpoint_y / scale;

    let mut x_ticks_path = Path::new();

    for i in 1..num_x_ticks {
        let pos_x = (midpoint_x + scale * i) as f32;
        let neg_x = (midpoint_x - scale * i) as f32;

        x_ticks_path.move_to(pos_x, 0.);
        // x_ticks_path.move_to(pos_x, (midpoint_y - tick_size / 2) as f32);
        x_ticks_path.line_to(pos_x, size.height as f32);
        // x_ticks_path.line_to(pos_x, (midpoint_y + tick_size / 2) as f32);

        x_ticks_path.move_to(neg_x, 0.);
        x_ticks_path.line_to(neg_x, size.height as f32);
    }

    let mut y_ticks_path = Path::new();

    for i in 1..num_y_ticks {
        let pos_y = (midpoint_y + scale * i) as f32;
        let neg_y = (midpoint_y - scale * i) as f32;

        y_ticks_path.move_to(0., pos_y);
        y_ticks_path.line_to(size.width as f32, pos_y);

        y_ticks_path.move_to(0., neg_y);
        y_ticks_path.line_to(size.width as f32, neg_y);
    }

    return (x_ticks_path, y_ticks_path);
}

impl Graph {
    fn init_graph<T: Renderer>(self, canvas: &mut Canvas<T>) {
        let size = self.size;

        let mut x_axis = Path::new();
        let mut y_axis = Path::new();
        let midpoint_vert = (size.height / 2) as f32;
        let midpoint_horiz = (size.width / 2) as f32;

        x_axis.move_to(0., midpoint_vert);
        x_axis.line_to(size.width as f32, midpoint_vert);

        y_axis.move_to(midpoint_horiz, 0.);
        y_axis.line_to(midpoint_horiz, size.height as f32);

        let (x_ticks, y_ticks) = tick_marks(size, self.scale);
        // TODO maybe create struct for tick marks

        let green_paint = Paint::color(Color::rgb(0, 255, 0));
        let axes_paint = green_paint.clone().with_line_width(3.);

        canvas.stroke_path(&x_ticks, &green_paint);
        canvas.stroke_path(&y_ticks, &green_paint);

        canvas.stroke_path(&x_axis, &axes_paint);
        canvas.stroke_path(&y_axis, &axes_paint);
    }
}

fn render_canvas<T: Renderer>(window: &Window, canvas: &mut Canvas<T>) {
    // Make sure the canvas has the right size:
    let size = window.inner_size();
    canvas.set_size(size.width, size.height, window.scale_factor() as f32);

    // clear canvas by filling with black
    canvas.clear_rect(0, 0, size.width, size.height, Color::black());

    let graph1 = Graph { size, scale: 20 };
    graph1.init_graph(canvas);
}

fn render<T: Renderer>(
    context: &PossiblyCurrentContext,
    surface: &Surface<WindowSurface>,
    window: &Window,
    canvas: &mut Canvas<T>,
    square_position: PhysicalPosition<f64>,
) {
    render_canvas(window, canvas);

    // Make smol red rectangle
    canvas.clear_rect(
        square_position.x as u32,
        square_position.y as u32,
        30,
        30,
        Color::rgbf(1., 0., 0.),
    );

    // Tell renderer to execute all drawing commands
    canvas.flush();

    // Display what we've just rendered
    surface
        .swap_buffers(context)
        .expect("Could not swap buffers");
}
