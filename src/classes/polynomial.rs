use super::traits::{Calculus, Expression};
use super::term::Term;

pub struct Polynomial {
  pub terms: Vec<Box<Term>>,
}

impl Maf for Polonomial {}

impl Polynomial {
  pub fn new(terms: Vec<Box<Term>>) -> Box<Polynomial> {
    Box::new(Polynomial { terms })
  }
}

impl Calculus for Polynomial {
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

impl Expression for Polynomial {
    fn evaluate(&self, v: f64) -> f64 {
        self.terms
            .iter()
            .map(|x| x.evaluate(v))
            .sum()
    }
}

impl std::fmt::Display for Polynomial {
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
