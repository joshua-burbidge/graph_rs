pub struct Equation {
    pub a: f32,
    pub b: f32,
}

pub trait Calculate {
    fn calc(&self, x: f32) -> f32;
}

impl Calculate for Equation {
    fn calc(&self, x: f32) -> f32 {
        self.a * x + self.b
    }
}
