pub trait Calculus {
    fn integral(&self) -> Box<dyn Maf>;
    fn derivative(&self) -> Box<dyn Maf>;
}

pub trait Expression {
    fn evaluate(&self, x: f64) -> f64;
}

pub trait Maf: Expression + Calculus + std::fmt::Display + std::ops::Neg<Output=Self> where Self: std::marker::Sized {}
