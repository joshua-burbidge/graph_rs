use application::{femtovg_init, parser};
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
        .plus_x_times(1.)
        .plus_const(-1.)
        .build();
    let neg_quad: Polynomial = PolynomialBuilder::new()
        .plus_x_squared_times(-0.5)
        .plus_x_times(1.)
        .plus_const(-1.)
        .build();

    let cubic: Polynomial = PolynomialBuilder::new()
        .plus_x_cubed_times(0.01)
        .plus_x_squared_times(-0.2)
        .plus_x_times(1.)
        .plus_const(0.)
        .build();

    let _poly: Polynomial = PolynomialBuilder::new()
        .add_term(Term::x_to_the(6).times(0.5))
        .plus_x_4th_times(-4.)
        .plus_x_squared_times(3.)
        .plus_const(-1.)
        .build();
    let s = PolynomialBuilder::new()
        .plus_x_4th_times(1.)
        .plus_x_4th_times(-1.5)
        .plus_x_4th_times(-2.)
        .plus_x_cubed_times(1.)
        .plus_x_cubed_times(2.)
        .plus_const(99.9)
        .plus_const(-99.8)
        .build();

    vec![linear, quad, neg_quad, cubic, s]
}

fn is_wasm() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::log_1(&format!("Using wasm").into());
        true
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}

fn main() {
    let equations = if is_wasm() || parser::has_demo_arg() {
        demo_equations()
    } else {
        parser::get_input()
    };

    println!("Graphing equations:");
    for e in &equations {
        println!("{e}");
    }

    let event_loop = EventLoop::new().expect("failed to create event loop");

    let mut app = femtovg_init::init_canvas(&event_loop, equations);

    event_loop.run_app(&mut app).expect("run failed");
}
