use super::Calculus;
use super::Term;

pub struct Polynomial {
  pub terms: Vec<Box<Term>>,
}

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
        .collect::<Vec<Box<Term>>>(),
    )
  }

  fn derivative(&self) -> Box<Self> {
    Polynomial::new(
      self
        .terms
        .iter()
        .map(|x| x.derivative())
        .collect::<Vec<Box<Term>>>(),
    )
  }
}

impl std::fmt::Display for Polynomial {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if self.terms.len() == 0 {
      write!(f, "0")
    } else {
      Ok(for term in 0..self.terms.len() {
        if term == 0 {
          write!(f, "{:#}", self.terms[term])?
        } else {
          write!(f, " + {:#}", self.terms[term])?
        }
      })
    }
  }
}
