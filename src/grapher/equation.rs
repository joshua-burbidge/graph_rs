pub struct Equation {
    pub a: f32,
    pub b: f32,
}

impl Equation {
    pub fn calc(&self, x: f32) -> f32 {
        self.a * x + self.b
    }
}
