use regex::Regex;
use std::{env, io};

use crate::grapher::equation::{Polynomial, Term};

pub fn parse_equation() -> Polynomial {
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

    println!("Parsed equation: {}", poly);

    poly
}

pub fn has_demo_arg() -> bool {
    let args: Vec<String> = env::args().collect();

    args.len() >= 2 && &args[1] == "--demo"
}
