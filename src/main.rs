use application::{femtovg_init, handler::MyApplicationHandler};
use grapher::equation::{Polynomial, PolynomialBuilder, Term};
use regex::Regex;
use std::{
    env,
    fmt::Display,
    io::{self, Write},
};
use winit::event_loop::EventLoop;

pub mod application;
pub mod grapher;

fn parse_equation() -> Polynomial {
    let prompt = "Enter polynomial in the form: 4.2x^2 -2x +0.4";
    println!("{prompt}");
    let mut polystring = String::new();

    io::stdin()
        .read_line(&mut polystring)
        .expect("Failed to read line");

    // requires no space between sign and term
    let regex = Regex::new(r"([+-]?(\d+(\.\d+)?)?)(x\^?(\d+)?)?").unwrap();

    let mut terms = Vec::new();

    for cap in regex.captures_iter(&polystring) {
        // println!("{:?}", cap);
        let whole_term = cap.get(0).unwrap().as_str();
        if whole_term.is_empty() {
            continue;
        }

        let coeff_opt = cap.get(1);
        let coeff = match coeff_opt {
            None => 1.,
            Some(coeff_match) => match coeff_match.as_str() {
                "-" => -1.,
                "+" => 1.,
                coeff_str => coeff_str.parse::<f32>().expect(&format!(
                    "invalid coefficient {}, in term: {}",
                    coeff_str, whole_term
                )),
            },
        };

        let power_term = cap.get(5);
        let x_exp_term = cap.get(4);

        let p = match power_term {
            None => match x_exp_term {
                None => 0,
                Some(x_exp_match) => {
                    if x_exp_match.as_str() == "x" {
                        1
                    } else {
                        panic!("invalid term, {}", whole_term);
                    }
                }
            },
            Some(power_match) => {
                let pow_string = power_match.as_str();
                pow_string.parse::<i32>().expect(&format!(
                    "invalid power: {}, in term: {}",
                    pow_string, whole_term
                ))
            }
        };

        terms.push(Term::new(coeff, p));
    }

    println!("Parsed terms: {:?}", terms);

    let poly = Polynomial::new(terms);

    println!("Equation: {}", poly);

    poly
}

fn has_demo_arg() -> bool {
    let args: Vec<String> = env::args().collect();

    args.len() >= 2 && &args[1] == "--demo"
}

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
    let equations = if has_demo_arg() {
        demo_equations()
    } else {
        vec![parse_equation()]
    };

    let event_loop = EventLoop::new().expect("failed to create event loop");

    let (context, canvas, window, surface) = femtovg_init::init_canvas(&event_loop);
    let default_scale = 50.;

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
