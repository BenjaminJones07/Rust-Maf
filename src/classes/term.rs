use super::calculus::Calculus;

pub struct Term {
  pub coef: f64, // negative coefficient makes the term is negative
  pub exp: f64,  // for sqrt use 1/2
}

impl Term {
  pub fn new(coef: f64, exp: f64) -> Term {
    Term { coef, exp }
  }
}

impl Calculus for Term {
  fn integral(&self) -> Self {
    Term::new(self.coef / (self.exp + 1f64), self.exp + 1f64)
  }

  fn derivative(&self) -> Self {
    Term::new(self.coef * self.exp, self.exp - 1f64)
  }
}