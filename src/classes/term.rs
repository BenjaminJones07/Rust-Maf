pub struct Term {
  pub coef: f64, // negative coefficient makes the term is negative
  pub exp: f64,  // for sqrt use 1/2
}

impl Term {
  pub fn new(coef: f64, exp: f64) -> Term {
    Term { coef, exp }
  }
}
