pub struct Polynomial {
  pub terms: (box Term)[],
}


impl Polynomial {
  pub fn new(terms: (box Term)[]) -> Polynomial {
    Polynomial {
      terms
    }
  }
}
