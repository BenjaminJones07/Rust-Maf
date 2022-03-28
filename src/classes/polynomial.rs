use super::calculus::Calculus;
use super::term::Term;

pub struct Polynomial {
  pub terms: Vec<Box<Term>>,
}

impl Polynomial {
  pub fn new(terms: Vec<Box<Term>>) -> Polynomial {
    Polynomial {
      terms
    }
  }
}
