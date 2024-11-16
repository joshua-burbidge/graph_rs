use regex::Regex;
use std::{env, io};

use crate::grapher::equation::{Polynomial, Term};

pub fn parse_equation() -> Polynomial {
    let prompt = "Enter polynomial in the form: 4.2x^2 - 2x + 0.4 (whitespace ignored, exponents must be integers)";
    println!("{prompt}");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let polystring: String = input.split_whitespace().collect();

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

pub fn has_demo_arg() -> bool {
    let args: Vec<String> = env::args().collect();

    args.len() >= 2 && &args[1] == "--demo"
}
