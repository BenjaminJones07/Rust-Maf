use super::{Calculus, Expression, miniMaf, Maf, Term};

pub enum VarFunc {
    x,
    sin(Box<dyn Maf<Output=VarFunc>>),
    cos(Box<dyn Maf<Output=VarFunc>>),
    tan(Box<dyn Maf<Output=VarFunc>>),
    cot(Box<dyn Maf<Output=VarFunc>>),
    sec(Box<dyn Maf<Output=VarFunc>>),
    csc(Box<dyn Maf<Output=VarFunc>>)
}

impl miniMaf for VarFunc {}
impl Maf for VarFunc {}

impl Calculus for VarFunc {
    fn derivative(&self) -> Box<Term> {
        match self { // TODO: Implement differentiation of inner v term
            VarFunc::x      => Term::new(0f64, vec![Box::new(VarFunc::x)], 0f64),
            VarFunc::sin(v) => Term::new(1f64, vec![Box::new(VarFunc::cos(*v))], 1f64),
            VarFunc::cos(v) => Term::new(-1f64, vec![Box::new(VarFunc::sin(*v))], 1f64),
            VarFunc::tan(v) => Term::new(1f64, vec![Box::new(VarFunc::sec(*v))], 2f64),
            VarFunc::cot(v) => Term::new(-1f64, vec![Box::new(VarFunc::csc(*v))], 2f64),
            VarFunc::sec(v) => Term::new(1f64, vec![Box::new(VarFunc::tan(*v)), Box::new(VarFunc::sec(*v))], 1f64),
            VarFunc::csc(v) => Term::new(-1f64, vec![Box::new(VarFunc::cot(*v)), Box::new(VarFunc::csc(*v))], 1f64)
        }
    }

    fn integral(&self) -> Box<Term> {
        match self {
            VarFunc::x      => Term::new(0.5, vec![Box::new(VarFunc::x)], 2f64),
            _               => Term::new(0f64, vec![Box::new(VarFunc::x)], 0f64)
        }
    }
}

impl Expression for VarFunc {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            VarFunc::x      => x,
            VarFunc::sin(v) => v.evaluate(x).sin(),
            VarFunc::cos(v) => v.evaluate(x).cos(),
            VarFunc::tan(v) => v.evaluate(x).tan(),
            VarFunc::cot(v) => v.evaluate(x).tan().powf(-1f64),
            VarFunc::sec(v) => v.evaluate(x).cos().powf(-1f64),
            VarFunc::csc(v) => v.evaluate(x).sin().powf(-1f64)
        }
    }
}

impl std::fmt::Display for VarFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VarFunc::x      => write!(f, "x"),
            VarFunc::sin(v) => write!(f, "sin({:#})", v),
            VarFunc::cos(v) => write!(f, "cos({:#})", v),
            VarFunc::tan(v) => write!(f, "tan({:#})", v),
            VarFunc::cot(v) => write!(f, "cot({:#})", v),
            VarFunc::sec(v) => write!(f, "sec({:#})", v),
            VarFunc::csc(v) => write!(f, "csc({:#})", v)
        }
    }
}

impl std::ops::Neg for VarFunc {
    type Output = Box<dyn miniMaf>;
  
    fn neg(self) -> Box<Term> {
        match self {
            VarFunc::x      => Term::new(-1f64, vec![Box::new(VarFunc::x)], 1f64),
            VarFunc::sin(v) => Term::new(-1f64, vec![Box::new(VarFunc::sin(v))], 1f64),
            VarFunc::cos(v) => Term::new(-1f64, vec![Box::new(VarFunc::sin(v))], 1f64),
            VarFunc::tan(v) => Term::new(-1f64, vec![Box::new(VarFunc::sin(v))], 1f64),
            VarFunc::cot(v) => Term::new(-1f64, vec![Box::new(VarFunc::sin(v))], 1f64),
            VarFunc::sec(v) => Term::new(-1f64, vec![Box::new(VarFunc::sin(v))], 1f64),
            VarFunc::csc(v) => Term::new(-1f64, vec![Box::new(VarFunc::sin(v))], 1f64)
        }
    }
  }

