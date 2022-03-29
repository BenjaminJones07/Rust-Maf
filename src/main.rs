mod classes;
mod syntax;

use classes::{Calculus};
use syntax::parse;

fn main() {
    let t = parse("0 + 1 + x + 2x^2".to_string());
    println!("{:#}", t);
    println!("{:#}", t.derivative());
    println!("{:#}", t.integral());
}
