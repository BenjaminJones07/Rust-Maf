use super::traits::{Calculus, Expression};
use super::term::Term;

pub struct Polynomial<T>
where
  T: Expression + Calculus + std::fmt::Display + std::ops::Neg
{
  pub terms: Vec<Box<Term<T>>>,
}

impl Maf for Polonomial {}

impl<T> Polynomial<T>
where
  T: Expression + Calculus + std::fmt::Display + std::ops::Neg
{
  pub fn new(terms: Vec<Box<Term<T>>>) -> Box<Polynomial<T>> {
    Box::new(Polynomial { terms })
  }
}

impl<T> Calculus for Polynomial<T>
where
  T: Expression + Calculus + std::fmt::Display + std::ops::Neg
{
  fn integral(&self) -> Box<Self> {
    Polynomial::new(
      self
        .terms
        .iter()
        .map(|x| x.integral())
        .collect::<Vec<Box<Term<T>>>>(),
    )
  }

  fn derivative(&self) -> Box<Self> {
    Polynomial::new(
      self
        .terms
        .iter()
        .map(|x| x.derivative())
        .collect::<Vec<Box<Term<T>>>>(),
    )
  }
}

impl<T> Expression for Polynomial<T>
where
  T: Expression + Calculus + std::fmt::Display + std::ops::Neg
{
    fn evaluate(&self, v: f64) -> f64 {
        self.terms
            .iter()
            .map(|x| x.evaluate(v))
            .sum()
    }
}

impl<T> std::fmt::Display for Polynomial<T>
where
  T: Expression + Calculus + std::fmt::Display + std::ops::Neg
{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if self.terms.len() == 0 {
      write!(f, "0")
    } else {
      let mut s = String::new();
      for term in 0..self.terms.len() {
        if term == 0 {
          //write!(s, "{:#}", self.terms[term]);
          s.push_str(format!("{:#}", self.terms[term]).as_str());
        } else {
          s.push_str(format!(" + {:#}", self.terms[term]).as_str());
        }
      }
      write!(f, "{}", s)
    }
  }
}
