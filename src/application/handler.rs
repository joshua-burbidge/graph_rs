use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, Paint, Path, Renderer};
use glutin::context::PossiblyCurrentContext;
use glutin::surface::Surface;
use glutin::{prelude::*, surface::WindowSurface};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, NamedKey};
use winit::window::Window;
use winit::window::WindowId;

use crate::application::femtovg_init;

#[derive(Default)]
pub struct MyApplicationHandler {
    close_requested: bool,
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

// enum Axis {
//     X,
//     Y,
// }
// struct TickMark {
//     size: PhysicalSize<u32>,
//     num: i32,
//     axis: Axis,
// }
struct Graph {
    size: PhysicalSize<u32>,
    scale: i32,
}

fn tick_marks(size: PhysicalSize<u32>, scale: i32) -> (Path, Path) {
    let midpoint_x = (size.width / 2) as i32;
    let midpoint_y = (size.height / 2) as i32;

    let num_x_ticks = midpoint_x / scale + 1;
    let num_y_ticks = midpoint_y / scale + 1;

    let mut x_ticks_path = Path::new();

    for i in 1..num_x_ticks {
        let pos_x = (midpoint_x + scale * i) as f32;
        let neg_x = (midpoint_x - scale * i) as f32;

        x_ticks_path.move_to(pos_x, 0.);
        x_ticks_path.line_to(pos_x, size.height as f32);

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
    fn init_graph<T: Renderer>(&self, canvas: &mut Canvas<T>) {
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

        let green_paint = Paint::color(Color::rgb(0, 255, 0)).with_line_width(0.5);
        let axes_paint = green_paint.clone().with_line_width(3.);

        canvas.stroke_path(&x_ticks, &green_paint);
        canvas.stroke_path(&y_ticks, &green_paint);

        canvas.stroke_path(&x_axis, &axes_paint);
        canvas.stroke_path(&y_axis, &axes_paint);
    }

    fn zero_zero_px(&self) -> (f32, f32) {
        let zero_x_px: f32 = (self.size.width / 2) as f32;
        let zero_y_px: f32 = (self.size.height / 2) as f32;
        (zero_x_px, zero_y_px)
    }

    // get x range in units, returns the first int greater than the screen size
    fn get_x_range(&self) -> (i32, i32) {
        let midpoint_x = (self.size.width / 2) as i32;
        let num_x_ticks = midpoint_x / self.scale + 1;

        let min_x = num_x_ticks * -1;
        let max_x = num_x_ticks;

        (min_x, max_x)
    }
    // fn get_min_y(self) -> i32 {}

    fn convert_point_to_px(&self, point: Point) -> (f32, f32) {
        let zero_zero = self.zero_zero_px();
        let (zero_x, zero_y) = zero_zero;

        let position_x = zero_x + (point.x * self.scale as f32);
        let position_y = zero_y - (point.y * self.scale as f32);

        (position_x, position_y)
    }

    fn graph_linear<T: Renderer>(self, equation: Equation, canvas: &mut Canvas<T>) {
        let (min_x, max_x) = self.get_x_range();

        let mut eq_path = Path::new();

        // works because it's linear, general case would be "in min_x..(max_x + 1)"
        for i in [min_x, max_x] {
            let point = Point {
                x: i as f32,
                y: equation.calc(i as f32),
            };
            let point_px = self.convert_point_to_px(point);

            if eq_path.is_empty() {
                eq_path.move_to(point_px.0, point_px.1);
            } else {
                eq_path.line_to(point_px.0, point_px.1);
            }
        }

        let red_paint = Paint::color(Color::rgb(255, 0, 0));
        canvas.stroke_path(&eq_path, &red_paint);
    }
}
// TODO point class that translates point to pixel
struct Point {
    x: f32,
    y: f32,
}

struct Equation {
    a: f32,
    b: f32,
}

impl Equation {
    fn calc(&self, x: f32) -> f32 {
        self.a * x + self.b
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

    let eq1 = Equation { a: 0.5, b: -1. };
    graph1.graph_linear(eq1, canvas);
}

fn render<T: Renderer>(
    context: &PossiblyCurrentContext,
    surface: &Surface<WindowSurface>,
    window: &Window,
    canvas: &mut Canvas<T>,
) {
    render_canvas(window, canvas);

    // Tell renderer to execute all drawing commands
    canvas.flush();

    // Display what we've just rendered
    surface
        .swap_buffers(context)
        .expect("Could not swap buffers");
}
