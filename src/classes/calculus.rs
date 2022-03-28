pub trait Calculus {
    fn integral(&self) -> Self;
    fn derivative(&self) -> Self;
}
