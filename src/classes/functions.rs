
use super::{Calculus, Expression, Maf, Term};

pub enum Trig {
    Sin(Box<dyn Maf>),
    Cos(Box<dyn Maf>),
    Tan(Box<dyn Maf>),
    Cot(Box<dyn Maf>),
    Sec(Box<dyn Maf>),
    Csc(Box<dyn Maf>),
}

impl Maf for Trig {
    fn neg(&self) -> Box<dyn Maf> {
        match self {
            Trig::Sin(x) => Term::new(-Box::new(Trig::Cos(x)), 1f64),
            Trig::Cos(x) => Box::new(Trig::Sin(x.neg())),
            Trig::Tan(x) => Box::new(Trig::Cot(x.neg())),
            Trig::Cot(x) => Box::new(Trig::Tan(x.neg())),
            Trig::Sec(x) => Box::new(Trig::Csc(x.neg())),
            Trig::Csc(x) => Box::new(Trig::Sec(x.neg())),
        }
    }

    fn cloned(&self) -> Box<dyn Maf> {
        match self {
            Trig::Sin(x) => Box::new(Trig::Sin(x.cloned())),
            Trig::Cos(x) => Box::new(Trig::Cos(x.cloned())),
            Trig::Tan(x) => Box::new(Trig::Tan(x.cloned())),
            Trig::Cot(x) => Box::new(Trig::Cot(x.cloned())),
            Trig::Sec(x) => Box::new(Trig::Sec(x.cloned())),
            Trig::Csc(x) => Box::new(Trig::Csc(x.cloned())),
        }
    }
}

impl Calculus for Trig {
    fn derivative(&self) -> Box<dyn Maf> {
        match self {
            Trig::Sin(v) => Term::new(
                1f64,
                vec![
                    Term::new(1f64, Box::new(Trig::Cos(v.cloned())), 1f64)
                ],
                1f64
            ),
            Trig::Cos(v) => Term::new(-1f64, vec![Tern::new(1f64, vec![Box::new(Trig::Sin(v.cloned()))], 1f64),
            Trig::Tan(v) => Term::new(1f64, vec![Term::new(1f64, vec![Box::new(Trig::Sec(v.cloned()))], 2f64)], 1f64),
            Trig::Cot(v) => Term::new(-1f64, vec![Term::new(1f64, vec![Box::new(Trig::Csc(v.cloned()))], 2f64)], 1f64),
            Trig::Sec(v) => Term::new(
                1f64,
                vec![
                    Box::new(Trig::Tan(v.cloned())),
                    Box::new(Trig::Sec(v.cloned())),
                ],
                1f64,
            ),
            Trig::Csc(v) => Term::new(
                1f64,
                vec![
                    Box::new(Trig::Csc(v.neg())),
                    Box::new(Trig::Cot(v.cloned())),
                ],
                1f64,
            ),
        }
    }

    fn integral(&self) -> Box<dyn Maf> {
        match self {
            Trig::Sin(v) => Term::new(-1f64, vec![Box::new(Trig::Cos(v.neg()))], 1f64),
            Trig::Cos(v) => Term::new(1f64, vec![Box::new(Trig::Sin(v.cloned()))], 1f64),
            Trig::Tan(_) => Term::new(0f64, vec![], 0f64),
            Trig::Cot(_) => Term::new(0f64, vec![], 0f64),
            Trig::Sec(_) => Term::new(0f64, vec![], 0f64),
            Trig::Csc(_) => Term::new(0f64, vec![], 0f64),
        }
    }
}

impl Expression for Trig {
    fn evaluate(&self, x: f64, y: f64, z: f64) -> f64 {
        match self {
            Trig::Sin(v) => v.evaluate(x, y, z).sin(),
            Trig::Cos(v) => v.evaluate(x, y, z).cos(),
            Trig::Tan(v) => v.evaluate(x, y, z).tan(),
            Trig::Cot(v) => v.evaluate(x, y, z).tan().powf(-1f64),
            Trig::Sec(v) => v.evaluate(x, y, z).cos().powf(-1f64),
            Trig::Csc(v) => v.evaluate(x, y, z).sin().powf(-1f64)
        }
    }
}

impl std::fmt::Display for Trig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Trig::Sin(v) => write!(f, "sin({:#})", *v),
            Trig::Cos(v) => write!(f, "cos({:#})", *v),
            Trig::Tan(v) => write!(f, "tan({:#})", *v),
            Trig::Cot(v) => write!(f, "cot({:#})", *v),
            Trig::Sec(v) => write!(f, "sec({:#})", *v),
            Trig::Csc(v) => write!(f, "csc({:#})", *v),
        }
    }
}