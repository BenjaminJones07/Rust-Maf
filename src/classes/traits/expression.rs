pub trait Expression {
    fn evaluate(&self, x: f64, y: f64, z: f64) -> f64;
}
