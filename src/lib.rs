mod classes;
use crate::classes::term::Term;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let t = Term::new();
        assert_eq!(1, 1)
    }
}