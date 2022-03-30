use super::{Calculus, Expression, Maf};

pub struct Term {
  pub coef: f64, // negative coefficient makes the term is negative
  pub vf: Vec<Box<dyn Maf>>,
  pub exp: f64,  // for sqrt use 1/2
}

impl Maf for Term {}

impl Term {
  pub fn new(coef: f64, vf: Vec<Box<dyn Maf>>, exp: f64) -> Box<Term<T>> {
    Box::new(Term { coef, vf, exp })
  }
}

impl Calculus for Term {
  fn integral(&self) -> Box<Term> {
    Term::new(
      self.coef / (self.exp + 1f64),
      (*self.vf).integral(),
      self.exp + 1f64
    ) // ax^n -> (a/(n+1))x^(n+1)
  }

  fn derivative(&self) -> Box<Term> {
    Term::new(
      self.coef * self.exp,
      self.vf.derivative(),
      self.exp - 1f64
    ) // ax^n -> anx^(n-1)
  }
}

impl Expression for Term {
    fn evaluate(&self, x: f64) -> f64 {
        self.coef * x.powf(self.exp)
    }
}

impl std::ops::Neg for Term {
  type Output = Box<Term<T>>;

  fn neg(self) -> Box<Term<T>> {
      Term::new(
          -self.coef,
          -*self.vf,
          -self.exp
      )
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
        write!(f, "{:#}", self.vf)
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
