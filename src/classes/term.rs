use super::traits::{Calculus, Expression};

pub struct Term {
  pub coef: f64, // negative coefficient makes the term is negative
  pub exp: f64,  // for sqrt use 1/2
}

impl Term {
  pub fn new(coef: f64, exp: f64) -> Box<Term> {
    Box::new(Term { coef, exp })
  }
}

impl Calculus for Term {
  fn integral(&self) -> Box<Self> {
    Term::new(self.coef / (self.exp + 1f64), self.exp + 1f64) // ax^n -> (a/(n+1))x^(n+1)
  }

  fn derivative(&self) -> Box<Self> {
    Term::new(self.coef * self.exp, self.exp - 1f64) // ax^n -> anx^(n-1)
  }
}

impl Expression for Term {
    fn evaluate(&self, x: f64) -> f64 {
        self.coef * x.powf(self.exp)
    }
}

impl std::fmt::Display for Term {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if self.coef == 0f64 {
      write!(f, "0")
    } else if self.coef == 1f64 {
      if self.exp == 0f64 {
        write!(f, "1")
      } else if self.exp == 1f64 {
        write!(f, "x")
      } else {
        write!(f, "x^{}", self.exp)
      }
    } else if self.exp == 0f64 {
      write!(f, "{}", self.coef)
    } else if self.exp == 1f64 {
      write!(f, "{}x", self.coef)
    } else {
      write!(f, "{}x^{}", self.coef, self.exp)
    }
  }
}

impl std::fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}
