use regex::Regex;
use std::{env, io};

use crate::{
    demo_equations,
    grapher::equation::{Equation, Polynomial, Term},
};

pub fn has_demo_arg() -> bool {
    let args: Vec<String> = env::args().collect();

    args.len() >= 2 && &args[1] == "--demo"
}

pub fn get_input() -> Vec<Equation> {
    let demo_or_custom = "[d] Graph a set of demo equations\n[e] Enter custom equations";
    println!("{demo_or_custom}");

    let mut demo_or_custom_loop = true;

    while demo_or_custom_loop {
        let mut demo_or_custom_input = String::new();
        io::stdin()
            .read_line(&mut demo_or_custom_input)
            .expect("Failed to read line");
        match demo_or_custom_input.chars().next() {
            Some('d') => {
                return demo_equations();
            }
            Some('e') => {
                demo_or_custom_loop = false;
            }
            _ => {
                println!("Invalid input. Try again.");
                demo_or_custom_loop = true;
            }
        };
    }

    get_custom_equations()
}

fn get_custom_equations() -> Vec<Equation> {
    let mut enter_another_equation = true;
    let mut equations: Vec<Equation> = Vec::new();

    while enter_another_equation {
        let eq = input_equation();
        equations.push(eq);

        println!("\nEntered equations:");
        for e in &equations {
            println!("{e}");
        }
        println!();

        let eq_prompt = "[e] Enter another equation";
        let graph_prompt = "[g] Graph these equations";

        print!("{}\n{}\n", eq_prompt, graph_prompt);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let first_char = input.chars().next();

        match first_char {
            Some('e') => {
                enter_another_equation = true;
            }
            Some('g') => {
                enter_another_equation = false;
            }
            _ => {
                println!("Invalid input. Try again.");
                enter_another_equation = true;
            }
        };
    }

    equations
}

fn input_equation() -> Equation {
    let prompt = "Enter polynomial in the form: 4.2x^2 - 2x + 0.4 (whitespace ignored, exponents must be integers)";
    println!("{prompt}");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    parse_equation(input)
}

fn parse_equation(equation_string: String) -> Polynomial {
    let polystring: String = equation_string.split_whitespace().collect();

    let regex = Regex::new(r"([+-]?(?:\d+(?:\.\d+)?)?)(x\^?(\d+)?)?").unwrap();
    // leads to an empty match at the very end
    let mut terms = Vec::new();

    for cap in regex.captures_iter(&polystring) {
        // println!("{:?}", cap);
        let whole_term_opt = cap.get(0); // eg, -4.2x^2
        let coeff_opt = cap.get(1); // eg, -4.2
        let x_exponential_opt = cap.get(2); // eg, x^2
        let power_opt = cap.get(3); // eg, 2

        let whole_term = whole_term_opt.unwrap().as_str();
        if whole_term.is_empty() {
            continue;
        }

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

        let p = match power_opt {
            None => match x_exponential_opt {
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
    // println!("Parsed terms: {:?}", terms);

    let poly = Polynomial::new(terms);
    println!("Parsed equation: {}", poly);

    poly
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_polynomial() {
        let input = String::from("0.5x^6 +1.234x^7 - 4x^4 + 3x^2 +x - 1");
        let expected = Polynomial::new(vec![
            Term::new(0.5, 6),
            Term::new(1.234, 7),
            Term::new(-4., 4),
            Term::new(3., 2),
            Term::new(1., 1),
            Term::new(-1., 0),
        ]);

        let result = parse_equation(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn leading_sign_neg() {
        let neg = String::from("-2.1x^3 - 1");
        let expected = Polynomial::new(vec![Term::new(-2.1, 3), Term::new(-1., 0)]);

        let result = parse_equation(neg);

        assert_eq!(result, expected);
    }

    #[test]
    fn leading_sign_pos() {
        let pos = String::from("+1.3x^2 + 2");
        let expected = Polynomial::new(vec![Term::new(1.3, 2), Term::new(2., 0)]);

        let result = parse_equation(pos);

        assert_eq!(result, expected);
    }

    #[test]
    fn handles_whitespace() {
        let input = String::from("   - \t  4.2x^3 +2x^2  - 3.7 ");
        let expected = Polynomial::new(vec![
            Term::new(-4.2, 3),
            Term::new(2., 2),
            Term::new(-3.7, 0),
        ]);

        let result = parse_equation(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn no_whitespace() {
        let input = String::from("5.2x^3-2x^2+1.9x-3");
        let expected = Polynomial::new(vec![
            Term::new(5.2, 3),
            Term::new(-2., 2),
            Term::new(1.9, 1),
            Term::new(-3., 0),
        ]);

        let result = parse_equation(input);

        assert_eq!(result, expected);
    }
}
