use super::super::traits::maf::Maf;

pub struct Term {
    pub coefficient: f64,
    pub content: Vec<Box<dyn Maf>>,
    pub exponent: f64,
}

impl Term {
    pub fn new(coefficient: f64, content: Vec<Box<dyn Maf>>, exponent: f64) -> Term {
        Term {
            coefficient,
            content,
            exponent,
        }
    }
}
