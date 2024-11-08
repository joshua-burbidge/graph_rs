use femtovg::{renderer::OpenGl, Canvas, Color, Paint, Path};
use winit::dpi::PhysicalSize;

use super::equation::{Calculate, Equation};

enum _Axis {
    X,
    Y,
}

// graph should be responsible for all paths and pixel conversions
// so that other structs can mathematical units
pub struct Graph<'a> {
    pub size: PhysicalSize<u32>,
    pub scale: i32,
    pub canvas: &'a mut Canvas<OpenGl>,
}

impl<'a> Graph<'a> {
    pub fn new(size: PhysicalSize<u32>, scale: i32, canvas: &'a mut Canvas<OpenGl>) -> Self {
        Graph {
            size,
            scale,
            canvas,
        }
    }

    pub fn init_graph(&mut self) {
        let size = self.size;

        let mut x_axis = Path::new();
        let mut y_axis = Path::new();
        let midpoint_vert = (size.height / 2) as f32;
        let midpoint_horiz = (size.width / 2) as f32;

        x_axis.move_to(0., midpoint_vert);
        x_axis.line_to(size.width as f32, midpoint_vert);

        y_axis.move_to(midpoint_horiz, 0.);
        y_axis.line_to(midpoint_horiz, size.height as f32);

        let (x_ticks, y_ticks) = self.tick_marks();

        let green_paint = Paint::color(Color::rgb(0, 255, 0)).with_line_width(0.5);
        let axes_paint = green_paint.clone().with_line_width(3.);

        self.canvas.stroke_path(&x_ticks, &green_paint);
        self.canvas.stroke_path(&y_ticks, &green_paint);

        self.canvas.stroke_path(&x_axis, &axes_paint);
        self.canvas.stroke_path(&y_axis, &axes_paint);
    }

    fn tick_marks(&self) -> (Path, Path) {
        let (min_x, max_x) = self.get_x_range();
        let (min_y, max_y) = self.get_y_range();

        let mut x_ticks_path = Path::new();
        let mut y_ticks_path = Path::new();

        for x in min_x..(max_x + 1) {
            let start_px = self.convert_point_to_px(Point::from_ints(x, min_y));
            let end_px = self.convert_point_to_px(Point::from_ints(x, max_y));

            x_ticks_path.move_to(start_px.0, start_px.1);
            x_ticks_path.line_to(end_px.0, end_px.1);
        }

        for y in min_y..(max_y + 1) {
            let start_px = self.convert_point_to_px(Point::from_ints(min_x, y));
            let end_px = self.convert_point_to_px(Point::from_ints(max_x, y));

            y_ticks_path.move_to(start_px.0, start_px.1);
            y_ticks_path.line_to(end_px.0, end_px.1);
        }

        (x_ticks_path, y_ticks_path)
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
    fn get_y_range(&self) -> (i32, i32) {
        let midpoint_y = (self.size.height / 2) as i32;
        let num_y_ticks = midpoint_y / self.scale + 1;

        let min_y = num_y_ticks * -1;
        let max_y = num_y_ticks;

        (min_y, max_y)
    }

    fn convert_point_to_px(&self, point: Point) -> (f32, f32) {
        let zero_zero = self.zero_zero_px();
        let (zero_x, zero_y) = zero_zero;

        let position_x = zero_x + (point.x * self.scale as f32);
        let position_y = zero_y - (point.y * self.scale as f32);

        (position_x, position_y)
    }

    pub fn graph_linear(&mut self, equation: Equation) {
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
        self.canvas.stroke_path(&eq_path, &red_paint);
    }
}

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn from_ints(x: i32, y: i32) -> Self {
        Point {
            x: x as f32,
            y: y as f32,
        }
    }
}
