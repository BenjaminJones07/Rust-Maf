use super::{Calculus, Expression, Maf};

pub struct Term {
  pub coef: f64, // negative coefficient makes the term is negative
  pub vf: Vec<Box<dyn Maf>>,
  pub exp: f64, // for sqrt use 1/2
}

impl Maf for Term {
  fn neg(&self) -> Box<dyn Maf> {
    Term::new(
      -self.coef,
      self.vf.iter().map(|x| x.neg()).collect(),
      -self.exp,
    )
  }

  fn cloned(&self) -> Box<dyn Maf> {
    Term::new(
      self.coef,
      self.vf.iter().map(|x| x.cloned()).collect(),
      self.exp,
    )
  }
}

impl Term {
  pub fn new(coef: f64, vf: Vec<Box<dyn Maf>>, exp: f64) -> Box<Term> {
    Box::new(Term { coef, vf, exp })
  }
}

impl Calculus for Term {
  fn integral(&self) -> Box<dyn Maf> {
    Term::new(
      self.coef / (self.exp + 1f64),
      self.vf.iter().map(|x| x.integral()).collect(),
      self.exp + 1f64,
    ) // ax^n -> (a/(n+1))x^(n+1)
  }

  fn derivative(&self) -> Box<dyn Maf> {
    Term::new(
      self.coef * self.exp,
      self.vf.iter().map(|x| x.derivative()).collect(),
      self.exp - 1f64,
    ) // ax^n -> anx^(n-1)
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
