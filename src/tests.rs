mod classes;

#[cfg(test)]
mod tests {
  use crate::classes::Term;

  #[test]
  fn test() {
    let t = Term::new(1f64, 1f64);
    println!("{:#}", t);
    assert_eq!(1, 1)
  }
}
