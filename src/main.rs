use application::{femtovg_init, handler::MyApplicationHandler, parser};
use grapher::equation::{Polynomial, PolynomialBuilder, Term};
use winit::event_loop::EventLoop;

mod application;
mod grapher;

fn demo_equations() -> Vec<Polynomial> {
    let linear = PolynomialBuilder::new()
        .plus_x_times(0.5)
        .plus_const(1.)
        .build();

    let quad: Polynomial = PolynomialBuilder::new()
        .plus_x_squared_times(0.5)
        .plus_x_times(0.)
        .plus_const(-1.)
        .build();

    let cubic: Polynomial = PolynomialBuilder::new()
        .plus_x_cubed_times(0.01)
        .plus_x_squared_times(-0.2)
        .plus_x_times(1.)
        .plus_const(0.)
        .build();

    let poly: Polynomial = PolynomialBuilder::new()
        .add_term(Term::x_to_the(6).times(0.5))
        .plus_x_4th_times(-4.)
        .plus_x_squared_times(3.)
        .plus_const(-1.)
        .build();
    vec![linear, quad, cubic, poly]
}

fn main() {
    let equations = if parser::has_demo_arg() {
        demo_equations()
    } else {
        parser::get_input()
    };

    println!("Graphing equations:");
    for e in &equations {
        println!("{e}");
    }

    let event_loop = EventLoop::new().expect("failed to create event loop");

    let (context, canvas, window, surface) = femtovg_init::init_canvas(&event_loop);
    let default_scale = 50.;

    window.focus_window();
    let mut app =
        MyApplicationHandler::new(window, context, surface, canvas, default_scale, equations);

    event_loop.run_app(&mut app).expect("run failed");
}

// let mut path = Path::new();
// let mut points = Path::new();

// let paint = Paint::color(Color::rgbf(1., 0., 0.));

// let c1 = (100., 300.);
// let c2 = (300., 300.);
// let start = (100., 100.);
// let end = (300., 100.);

// for point in [c1, c2, start, end] {
//     points.circle(point.0, point.1, 3.);
// }

// path.move_to(start.0, start.1);
// path.bezier_to(c1.0, c1.1, c2.0, c2.1, end.0, end.1);
// canvas.stroke_path(&path, &paint);
// canvas.fill_path(&points, &paint);
