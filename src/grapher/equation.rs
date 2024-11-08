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
