mod classes;
mod syntax;

use classes::Calculus;
use syntax::parse;

fn main() {
    let t = parse("0 + 1 + x + 2x^2".to_string());
    println!("Debug: ");
    println!("{:?}", t);
    println!("{:?}", t.derivative());
    println!("{:?}", t.integral());
    println!("\nPretty: ");
    println!("{:#}", t);
    println!("{:#}", t.derivative());
    println!("{:#}", t.integral());
}
