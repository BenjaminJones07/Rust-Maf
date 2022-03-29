mod classes;

#[cfg(test)]
mod tests {
  use crate::classes::{Calculus, Term, Polynomial;

  #[test]
  fn test1() {
    let t1 = Term::new(3f64, 4f64); // 3x^4
    let t2 = Term::new(12f64, 3f64); // 12x^3

    assert_eq!(format!("{:#}", *t1.derivative()), format!("{:#}", *t2))

  #[test]
  fn test2() {
    let t = Term::new(1f64, 1f64);
    println!("{:?}", t);
    assert_eq!(1, 1)
  }

  #[test]
  fn printing() { 
    let t = Polynomial::new(vec![
      Term::new(0f64, 0f64),
      Term::new(1f64, 0f64),
      Term::new(1f64, 1f64),
      Term::new(2f64, 2f64),
    ]);
    assert_eq!(format!("{:#}", t), "0 + 1 + x + 2x^2")
  }
}
