mod classes;

#[cfg(test)]
mod tests {
  use crate::classes::{Term, Calculus};

  #[test]
  fn test() {
    let t1 = Term::new(3f64, 4f64); // 3x^4
    let t2 = Term::new(12f64, 3f64); // 12x^3

    assert_eq!(format!("{:#}", *t1.derivative()), format!("{:#}", *t2))
  }
}
