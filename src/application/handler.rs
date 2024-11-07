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

        let green_paint = Paint::color(Color::rgb(0, 255, 0));
        let axes_paint = green_paint.clone().with_line_width(3.);

        canvas.stroke_path(&x_ticks, &green_paint);
        canvas.stroke_path(&y_ticks, &green_paint);

        canvas.stroke_path(&x_axis, &axes_paint);
        canvas.stroke_path(&y_axis, &axes_paint);
    }

    // fn get_min_x(self) -> i32 {}
    // fn get_min_y(self) -> i32 {}

    fn convert_point_to_px(&self, point: Point) -> (i32, i32) {
        let zero_zero = ((self.size.width / 2) as i32, (self.size.height / 2) as i32);
        let (zero_x, zero_y) = zero_zero;

        let position_x = zero_x + (point.x * self.scale);
        let position_y = zero_y - (point.y * self.scale);

        (position_x, position_y)
    }

    fn graph_linear<T: Renderer>(self, equation: Equation, canvas: &mut Canvas<T>) {
        // let point_interval = 10;
        // TODO compute min and max x and y values
        // loop through range

        let point_1 = Point {
            x: 0,
            y: equation.calc(0),
        };
        let point_2 = Point {
            x: 5,
            y: equation.calc(5),
        };

        let mut path = Path::new();

        let point_1_px = self.convert_point_to_px(point_1);
        let point_2_px = self.convert_point_to_px(point_2);
        path.move_to(point_1_px.0 as f32, point_1_px.1 as f32);
        path.line_to(point_2_px.0 as f32, point_2_px.1 as f32);

        let red_paint = Paint::color(Color::rgb(255, 0, 0));
        canvas.stroke_path(&path, &red_paint);
    }
}
// TODO point class that translates point to pixel
struct Point {
    x: i32,
    y: i32,
}

struct Equation {
    a: i32,
    b: i32,
}

impl Equation {
    fn calc(&self, x: i32) -> i32 {
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

    let eq1 = Equation { a: 2, b: 1 };
    graph1.graph_linear(eq1, canvas);
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
        (square_position.x - 5.) as u32,
        (square_position.y - 5.) as u32,
        10,
        10,
        Color::rgbf(1., 0., 0.),
    );

    // Tell renderer to execute all drawing commands
    canvas.flush();

    // Display what we've just rendered
    surface
        .swap_buffers(context)
        .expect("Could not swap buffers");
}
