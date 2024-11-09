enum _Equation {
    Linear,
    Quadratic,
}

pub struct Linear {
    pub a: f32,
    pub b: f32,
}

pub struct Quadratic {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

pub struct Cubic {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

#[derive(Default)]
pub struct Polynomial {
    terms: Vec<Term>,
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

impl Polynomial {
    pub fn new(terms: Vec<Term>) -> Self {
        Polynomial { terms }
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
        Polynomial { terms: self.terms }
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

pub struct Term {
    power: i32,
    c: f32,
}
impl Term {
    pub fn new(c: f32, power: i32) -> Self {
        Term { power, c }
    }
    pub fn plus_x_to_the(power: i32) -> Self {
        Term { power, c: 1. }
    }
    pub fn times(mut self, c: f32) -> Self {
        self.c = c;
        self
    }
}

pub trait Calculate {
    fn calc(&self, x: f32) -> f32;
}

impl Calculate for Linear {
    fn calc(&self, x: f32) -> f32 {
        self.a * x + self.b
    }
}

impl Calculate for Quadratic {
    fn calc(&self, x: f32) -> f32 {
        self.a * (x.powi(2)) + self.b * x + self.c
    }
}

impl Calculate for Cubic {
    fn calc(&self, x: f32) -> f32 {
        self.a * (x.powi(3)) + self.b * x.powi(2) + self.c * x + self.d
    }
}
