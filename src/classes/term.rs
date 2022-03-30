use super::{Calculus, Expression, miniMaf, Maf, VarFunc};

pub struct Term {
  pub coef: f64, // negative coefficient makes the term is negative
  pub vf: Vec<Box<dyn Maf>>,
  pub exp: f64,  // for sqrt use 1/2
}

impl miniMaf for Term {}
impl Maf for Term {}

impl Term {
  pub fn new(coef: f64, vf: Vec<Box<dyn Maf>>, exp: f64) -> Box<Term> {
    Box::new(Term { coef, vf, exp })
  }
}

impl Calculus for Term {
  fn integral(&self) -> Box<Term> {
    if self.vf.len() == 1 {
      Term::new(
        self.coef / (self.exp + 1f64),
        self.vf[0].integral(),
        self.exp + 1f64
      ) // ax^n -> (a/(n+1))x^(n+1)
    } else {
      Term::new(0, VarFunc::x, 0)
    }
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
  type Output = Box<dyn miniMaf>;

  fn neg(self) -> Box<Term> {
      Term::new(
          -self.coef,
          self.vf
              .iter()
              .map(|x| *x.neg())
              .collect::<Vec<Box<dyn Maf>>>(),
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
        write!(f, "{:#}", self.vf.iter().map(|x| format!("{:#}", x)).collect::<Vec<String>>().join(" * "))
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
