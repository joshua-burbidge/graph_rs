use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn from_ints(x: i32, y: i32) -> Self {
        Point {
            x: x as f32,
            y: y as f32,
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Polynomial {
    terms: Vec<Term>,
    precision: u32,
}

impl Polynomial {
    pub fn new(terms: Vec<Term>) -> Self {
        let default_precision = 4;

        Polynomial {
            terms,
            precision: default_precision,
        }
    }

    pub fn simplify(&self) -> Self {
        let mut simplified_terms = Vec::<Term>::new();

        for term in &self.terms {
            // if the term for this power has already been collected, continue
            if simplified_terms
                .iter()
                .find(|t| t.power == term.power)
                .is_some()
            {
                continue;
            }
            let this_power_terms = self.terms.iter().filter(|t| t.power == term.power);
            let combined_c = this_power_terms.fold(0., |acc: f32, t: &Term| acc + t.c);

            let rounding_factor = 10_i32.pow(self.precision) as f32;
            let rounded_c = (combined_c * rounding_factor).round() / rounding_factor;
            simplified_terms.push(Term::new(rounded_c, term.power));
        }

        Polynomial::new(simplified_terms)
    }

    fn _find_term(&self, power: i32) -> Term {
        let term_opt = self.terms.iter().find(|t| t.power == power);
        match term_opt {
            None => Term::new(0., power),
            Some(term) => Term::new(term.c, term.power),
        }
    }

    pub fn _find_vertex(&self) -> Point {
        let a = self._find_term(2).c;
        let b = self._find_term(1).c;

        let v_x = -b / (2. * a);
        let v_y = self.calc(v_x);

        Point { x: v_x, y: v_y }
    }
}

// TODO test this

// impl PartialEq for Polynomial
// so that term order doesn't matter

pub trait Calculate: Debug + Display {
    fn calc(&self, x: f32) -> f32;
}

impl Calculate for Polynomial {
    fn calc(&self, x: f32) -> f32 {
        let mut sum = 0.;

        for term in &self.terms {
            let term_value = term.c * x.powi(term.power);
            sum += term_value;
        }

        sum
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "y = ").expect("failed to write");
        for item in self.terms.iter() {
            // Print each element
            write!(f, "{} ", item)?;
        }
        Ok(())
    }
}

pub trait CouldBeLinear {
    fn is_linear(&self) -> bool;
}

impl CouldBeLinear for Polynomial {
    fn is_linear(&self) -> bool {
        for term in &self.terms {
            if term.power > 1 {
                return false;
            }
        }

        true
    }
}

pub struct PolynomialBuilder {
    terms: Vec<Term>,
}

impl PolynomialBuilder {
    pub fn new() -> Self {
        PolynomialBuilder {
            terms: Vec::<Term>::new(),
        }
    }
    pub fn add_term(mut self, term: Term) -> Self {
        self.terms.push(term);
        self
    }
    pub fn build(self) -> Polynomial {
        Polynomial::new(self.terms).simplify()
    }
    pub fn plus_const(self, coeff: f32) -> Self {
        self.add_term(Term { c: coeff, power: 0 })
    }
    pub fn plus_x_times(self, coeff: f32) -> Self {
        self.add_term(Term { c: coeff, power: 1 })
    }
    pub fn plus_x_squared_times(self, coeff: f32) -> Self {
        self.add_term(Term { c: coeff, power: 2 })
    }
    pub fn plus_x_cubed_times(self, coeff: f32) -> Self {
        self.add_term(Term { c: coeff, power: 3 })
    }
    pub fn plus_x_4th_times(self, coeff: f32) -> Self {
        self.add_term(Term { c: coeff, power: 4 })
    }
}

#[derive(Debug, PartialEq)]
pub struct Term {
    power: i32,
    c: f32,
}
impl Term {
    pub fn new(c: f32, power: i32) -> Self {
        Term { power, c }
    }
    pub fn x_to_the(power: i32) -> Self {
        Term { power, c: 1. }
    }
    pub fn times(mut self, c: f32) -> Self {
        self.c = c;
        self
    }
}
impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = if self.c.is_sign_positive() { "+" } else { "-" };
        let x_power = match self.power {
            0 => String::from(""),
            1 => String::from("x"),
            _ => format!("x^{}", self.power),
        };
        write!(f, "{} {}{}", sign, self.c.abs(), x_power)
    }
}
