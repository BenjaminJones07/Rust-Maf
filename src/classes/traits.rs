pub trait Calculus {
    fn integral(&self) -> Box<dyn Maf>;
    fn derivative(&self) -> Box<dyn Maf>;
}

pub trait Expression {
    fn evaluate(&self, x: f64) -> f64;
}

pub trait miniMaf: Expression + Calculus + std::fmt::Display {}
pub trait Maf: miniMaf + std::ops::Neg<Output=Box<dyn miniMaf>> {}
