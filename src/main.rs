mod classes;

use classes::{Polynomial, Term};

fn main() {
  let t = Polynomial::new(vec![
    Term::new(0f64, 0f64),
    Term::new(1f64, 0f64),
    Term::new(1f64, 1f64),
    Term::new(2f64, 2f64),
    Term::new(3f64, 3f64),
  ]);
  println!("{:#}", t);
}
