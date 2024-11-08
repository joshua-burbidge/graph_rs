use femtovg::{renderer::OpenGl, Canvas, Color, Paint, Path};
use winit::dpi::PhysicalSize;

use super::equation::{Calculate, Linear};

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
        let mut axes = Path::new();

        let (min_x, max_x) = self.get_x_range();
        let (min_y, max_y) = self.get_y_range();

        let x_axis_start = self.convert_point_to_px(Point::from_ints(min_x, 0));
        let x_axis_end = self.convert_point_to_px(Point::from_ints(max_x, 0));

        let y_axis_start = self.convert_point_to_px(Point::from_ints(0, min_y));
        let y_axis_end = self.convert_point_to_px(Point::from_ints(0, max_y));

        axes.move_to(x_axis_start.0, x_axis_start.1);
        axes.line_to(x_axis_end.0, x_axis_start.1);

        axes.move_to(y_axis_start.0, y_axis_start.1);
        axes.line_to(y_axis_end.0, y_axis_end.1);

        self.tick_marks();

        let axes_paint = Paint::color(Color::rgb(0, 255, 0)).with_line_width(3.);

        self.canvas.stroke_path(&axes, &axes_paint);
    }

    fn tick_marks(&mut self) {
        let (min_x, max_x) = self.get_x_range();
        let (min_y, max_y) = self.get_y_range();

        let mut ticks_path = Path::new();
        let mut significant_ticks_path = Path::new();

        for x in min_x..(max_x + 1) {
            let start_px = self.convert_point_to_px(Point::from_ints(x, min_y));
            let end_px = self.convert_point_to_px(Point::from_ints(x, max_y));

            if x % 10 == 0 {
                significant_ticks_path.move_to(start_px.0, start_px.1);
                significant_ticks_path.line_to(end_px.0, end_px.1);
            } else {
                ticks_path.move_to(start_px.0, start_px.1);
                ticks_path.line_to(end_px.0, end_px.1);
            }
        }

        for y in min_y..(max_y + 1) {
            let start_px = self.convert_point_to_px(Point::from_ints(min_x, y));
            let end_px = self.convert_point_to_px(Point::from_ints(max_x, y));

            if y % 10 == 0 {
                significant_ticks_path.move_to(start_px.0, start_px.1);
                significant_ticks_path.line_to(end_px.0, end_px.1);
            } else {
                ticks_path.move_to(start_px.0, start_px.1);
                ticks_path.line_to(end_px.0, end_px.1);
            }
        }

        let green_paint = Paint::color(Color::rgb(0, 255, 0)).with_line_width(0.4);
        let wider_paint = green_paint.clone().with_line_width(0.6);
        self.canvas.stroke_path(&ticks_path, &green_paint);
        self.canvas
            .stroke_path(&significant_ticks_path, &wider_paint);
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

    pub fn graph_linear(&mut self, equation: Linear) {
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

    pub fn graph_poly<T: Calculate>(&mut self, equation: T) {
        let (min_x, max_x) = self.get_x_range();
        let points_per_unit = 100;

        let loop_bounds = ((min_x * points_per_unit), (max_x * points_per_unit));

        let mut eq_path = Path::new();

        for i in loop_bounds.0..loop_bounds.1 {
            let x = i as f32 / points_per_unit as f32;

            let point = Point {
                x,
                y: equation.calc(x),
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
