use super::{Calculus, Expression, Maf, Term};

pub enum VarFunc {
    Sin(Box<dyn Maf>),
    Cos(Box<dyn Maf>),
    Tan(Box<dyn Maf>),
    Cot(Box<dyn Maf>),
    Sec(Box<dyn Maf>),
    Csc(Box<dyn Maf>),
}

impl Maf for VarFunc {
    fn neg(&self) -> Box<dyn Maf> {
        match self {
            VarFunc::Sin(x) => Box::new(VarFunc::Cos(x.neg())),
            VarFunc::Cos(x) => Box::new(VarFunc::Sin(x.neg())),
            VarFunc::Tan(x) => Box::new(VarFunc::Cot(x.neg())),
            VarFunc::Cot(x) => Box::new(VarFunc::Tan(x.neg())),
            VarFunc::Sec(x) => Box::new(VarFunc::Csc(x.neg())),
            VarFunc::Csc(x) => Box::new(VarFunc::Sec(x.neg())),
        }
    }

    fn cloned(&self) -> Box<dyn Maf> {
        match self {
            VarFunc::Sin(x) => Box::new(VarFunc::Sin(x.cloned())),
            VarFunc::Cos(x) => Box::new(VarFunc::Cos(x.cloned())),
            VarFunc::Tan(x) => Box::new(VarFunc::Tan(x.cloned())),
            VarFunc::Cot(x) => Box::new(VarFunc::Cot(x.cloned())),
            VarFunc::Sec(x) => Box::new(VarFunc::Sec(x.cloned())),
            VarFunc::Csc(x) => Box::new(VarFunc::Csc(x.cloned())),
        }
    }
}

impl Calculus for VarFunc {
    fn derivative(&self) -> Box<dyn Maf> {
        match self {
            VarFunc::Sin(v) => Term::new(1f64, vec![Box::new(VarFunc::Cos(v.cloned()))], 1f64),
            VarFunc::Cos(v) => Term::new(1f64, vec![Box::new(VarFunc::Sin(v.neg()))], 1f64),
            VarFunc::Tan(v) => Term::new(
                1f64,
                vec![Box::new(VarFunc::Sec(Box::new(VarFunc::Sec(v.cloned()))))], // TODO: make this use a better method than sec in sec for sec^2
                1f64,
            ),
            VarFunc::Cot(v) => Term::new(
                1f64,
                vec![Box::new(VarFunc::Csc(Box::new(VarFunc::Csc(v.neg()))))], // TODO: make this use a better method than csc in csc for csc^2
                1f64,
            ),
            VarFunc::Sec(v) => Term::new(
                1f64,
                vec![
                    Box::new(VarFunc::Tan(v.cloned())),
                    Box::new(VarFunc::Sec(v.cloned())),
                ],
                1f64,
            ),
            VarFunc::Csc(v) => Term::new(
                1f64,
                vec![
                    Box::new(VarFunc::Csc(v.neg())),
                    Box::new(VarFunc::Cot(v.cloned())),
                ],
                1f64,
            ),
        }
    }

    fn integral(&self) -> Box<dyn Maf> {
        match self {
            VarFunc::Sin(v) => Term::new(1f64, vec![Box::new(VarFunc::Cos(v.neg()))], 1f64),
            VarFunc::Cos(v) => Term::new(1f64, vec![Box::new(VarFunc::Sin(v.cloned()))], 1f64),
            VarFunc::Tan(_) => Term::new(0f64, vec![], 0f64), // TODO: implement tan
            VarFunc::Cot(_) => Term::new(0f64, vec![], 0f64), // TODO: implement cot
            VarFunc::Sec(_) => Term::new(0f64, vec![], 0f64), // TODO: implement sec
            VarFunc::Csc(_) => Term::new(0f64, vec![], 0f64), // TODO: implement csc
        }
    }
}

impl Expression for VarFunc {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            VarFunc::Sin(v) => v.evaluate(x).sin(),
            VarFunc::Cos(v) => v.evaluate(x).cos(),
            VarFunc::Tan(v) => v.evaluate(x).tan(),
            VarFunc::Cot(v) => v.evaluate(x), // .cot(), // TODO: implement cot
            VarFunc::Sec(v) => v.evaluate(x), // .sec(), // TODO: implement sec
            VarFunc::Csc(v) => v.evaluate(x), // .csc(), // TODO: implement csc
        }
    }
}

impl std::fmt::Display for VarFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VarFunc::Sin(v) => write!(f, "sin({:#})", *v),
            VarFunc::Cos(v) => write!(f, "cos({:#})", *v),
            VarFunc::Tan(v) => write!(f, "tan({:#})", *v),
            VarFunc::Cot(v) => write!(f, "cot({:#})", *v),
            VarFunc::Sec(v) => write!(f, "sec({:#})", *v),
            VarFunc::Csc(v) => write!(f, "csc({:#})", *v),
        }
    }
}
