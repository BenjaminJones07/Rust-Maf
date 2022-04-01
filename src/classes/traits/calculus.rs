use super::maf::Maf;

pub trait Calculus {
    fn integral(&self) -> Box<dyn Maf>;
    fn derivative(&self) -> Box<dyn Maf>;
}
