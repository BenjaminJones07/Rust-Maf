mod classes;
mod syntax;

use classes::{Polynomial, Term};

fn parse(input: String) -> Box<Polynomial> {
  let mut parser = syntax::Parser::new(input);
  parser.parse()
}

fn main() {
  let t = Polynomial::new(vec![
    Term::new(0f64, 0f64),
    Term::new(1f64, 0f64),
    Term::new(1f64, 1f64),
    Term::new(2f64, 2f64),
  ]);
  println!("{:#}", t);
  let t = parse("0 + 1 + x + 2x^2".to_string());
  println!("{:#}", t);
}
