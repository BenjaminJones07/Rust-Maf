use super::{Calculus, Expression, Maf, Term};

pub enum VarFunc {
    sin(Box<dyn Maf>),
    cos(Box<dyn Maf>),
    tan(Box<dyn Maf>),
    cot(Box<dyn Maf>),
    sec(Box<dyn Maf>),
    csc(Box<dyn Maf>)
}

impl Maf for VarFunc {}

impl<T> Calculus for VarFunc<T> where T: Maf {
    fn derivative<T>(&self: &VarFunc<T>) -> Box<T>  {
        match self {
            VarFunc::sin(v) => Term::new(VarFunc::cos(v));
            VarFunc::cos(v) => Term::new();
            VarFunc::tan(v) => Term::new(VarFunc::cot(v));
            VarFunc::cot(v) => Term::new();
            VarFunc::sec(v) => Term::new(VarFunc::csc(v));
            VarFunc::csc(v) => Term::new();
        }
    }

    fn integral(&self) -> Box<Cosine<T>> {
        Cosine::new(-*self.term)
    }
}
