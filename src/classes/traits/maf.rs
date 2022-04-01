pub trait Maf: std::fmt::Display + std::fmt::Debug {
    fn cloned(&self) -> Box<dyn Maf>;
    fn negated(&self) -> Box<dyn Maf>;
}
