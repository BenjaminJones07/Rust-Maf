use super::{Calculus, Expression, Maf};

pub struct Term<T> {
  pub coef: f64, // negative coefficient makes the term is negative
  pub vf: Box<T>,
  pub exp: f64,  // for sqrt use 1/2
}

impl Maf for Term {}

impl<T> Term<T>
where
  T: Expression + Calculus + std::fmt::Display
{
  pub fn new(coef: f64, vf: Box<T>, exp: f64) -> Box<Term<T>> {
    Box::new(Term { coef, vf, exp })
  }
}

impl<T, V> Calculus for Term<T>
where
  T: Expression + Calculus + std::fmt::Display,
  V: Expression + Calculus + std::fmt::Display
{
  type DReturn = V;
  type IReturn = Term<V>;

  fn integral(&self) -> Box<Term<V>> {
    Term::new(
      self.coef / (self.exp + 1f64),
      (*self.vf).integral(),
      self.exp + 1f64
    ) // ax^n -> (a/(n+1))x^(n+1)
  }

  fn derivative(&self) -> Box<Self> {
    Term::new(
      self.coef * self.exp,
      self.vf.derivative(),
      self.exp - 1f64
    ) // ax^n -> anx^(n-1)
  }
}

impl<T> Expression for Term<T>
where
  T: Expression + Calculus + std::fmt::Display
{
    fn evaluate(&self, x: f64) -> f64 {
        self.coef * x.powf(self.exp)
    }
}

impl<T> std::ops::Neg for Term<T> where T: Expression + Calculus + std::fmt::Display + std::ops::Neg {
  type Output = Box<Term<T>>;

  fn neg(self) -> Box<Term<T>> {
      Term::new(
          -self.coef,
          -*self.vf,
          -self.exp
      )
  }
}

impl<T> std::fmt::Display for Term<T>
where
  T: Expression + Calculus + std::fmt::Display
{
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

impl<T> std::fmt::Debug for Term<T>
where
  T: Expression + Calculus + std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}
