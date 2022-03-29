pub trait Calculus {
    type DReturn;
    type IReturn;

    fn integral(&self) -> Box<Self::IReturn>;
    fn derivative(&self) -> Box<Self::DReturn>;
}

pub trait Expression {
    fn evaluate(&self, x: f64) -> f64;
}