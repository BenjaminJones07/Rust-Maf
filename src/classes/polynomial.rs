use super::{Calculus, Expression, Maf};

pub struct Polynomial {
  pub terms: Vec<Box<dyn Maf>>,
}

impl Maf for Polynomial {
  fn neg(&self) -> Box<dyn Maf> {
    Polynomial::new(
      self
        .terms
        .iter()
        .map(|x| x.neg())
        .collect::<Vec<Box<dyn Maf>>>(),
    )
  }

  fn cloned(&self) -> Box<dyn Maf> {
    Polynomial::new(
      self
        .terms
        .iter()
        .map(|x| x.cloned())
        .collect::<Vec<Box<dyn Maf>>>(),
    )
  }
}

impl Polynomial {
  pub fn new(terms: Vec<Box<dyn Maf>>) -> Box<Polynomial> {
    Box::new(Polynomial { terms })
  }
}

impl Calculus for Polynomial {
  fn integral(&self) -> Box<dyn Maf> {
    Polynomial::new(
      self
        .terms
        .iter()
        .map(|x| x.integral())
        .collect::<Vec<Box<dyn Maf>>>(),
    )
  }

  fn derivative(&self) -> Box<dyn Maf> {
    Polynomial::new(
      self
        .terms
        .iter()
        .map(|x| x.derivative())
        .collect::<Vec<Box<dyn Maf>>>(),
    )
  }
}

impl Expression for Polynomial {
  fn evaluate(&self, x: f64, y:f64, z: f64) -> f64 {
    self.terms.iter().map(|t| t.evaluate(x, y, z)).sum()
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
