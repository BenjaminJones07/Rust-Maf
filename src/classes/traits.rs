pub trait Calculus {
    fn integral(&self) -> Box<dyn Maf>;
    fn derivative(&self) -> Box<dyn Maf>;
}

pub trait Expression {
    fn evaluate(&self, x: f64, y: f64, z: f64) -> f64;
}

pub trait Maf: Expression + Calculus + std::fmt::Display + std::fmt::Debug {
    fn neg(&self) -> Box<dyn Maf>;
    fn reciprical(&self) -> Box<dyn Maf>;
    fn cloned(&self) -> Box<dyn Maf>;
}
