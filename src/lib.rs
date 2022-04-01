mod classes;

#[cfg(test)]
mod tests {
    use crate::classes::structures::term::Term;

    #[test]
    fn it_works() {
        Term::new(1f64, vec![], 1f64);
        assert_eq!(2 + 2, 4);
    }
}
