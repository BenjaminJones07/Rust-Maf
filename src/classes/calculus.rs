pub trait Calculus {
    fn integral(&self) -> Box<Self>;
    fn derivative(&self) -> Box<Self>;
}
