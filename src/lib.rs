mod classes;

#[cfg(test)]
mod tests {
  use crate::classes::{Term, Polynomial};

  #[test]
  fn test() {
    let t = Term::new(1f64, 1f64);
    println!("{:#}", t);
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
