use super::super::structures::term::Term;

pub struct Polynomial {
    pub terms: Vec<Term>,
}

impl Polynomial {
    pub fn new(terms: Vec<Term>) -> Polynomial {
        Polynomial { terms }
    }
}
