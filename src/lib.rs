mod classes;

#[cfg(test)]
mod tests {
  use crate::classes::{Calculus, Term, Polynomial, Sine, trigNewTrait};

  #[test]
  fn test1() {
    let t1 = Term::new(3f64, 4f64); // 3x^4 (Derivative: 12x^3)
    let t2 = Term::new(12f64, 3f64); // 12x^3

    assert_eq!(format!("{:#}", *t1.derivative()), format!("{:#}", *t2))
  }

  #[test]
  fn printing() { 
    let t = Polynomial::new(vec![
      Term::new(0f64, 0f64), //0
      Term::new(1f64, 0f64), //1
      Term::new(1f64, 1f64), //x
      Term::new(2f64, 2f64), //2x^2
    ]);
    assert_eq!(format!("{:#}", t), "0 + 1 + x + 2x^2")
  }

  #[test]
  fn trig() {
    let t = Term::new(1f64, 3f64);
    let s = Sine::new(t);
  }
}
