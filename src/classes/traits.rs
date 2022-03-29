pub trait Calculus {
    fn integral(&self) -> Box<Self>;
    fn derivative(&self) -> Box<Self>;
}

pub trait Expression {
    fn evaluate(&self, x: f64) -> f64;
}