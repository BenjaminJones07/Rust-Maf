use super::{Calculus, Expression, Maf, Term};

// Macro this!!!

// pub struct Sine<T> where T: Expression + Calculus + std::fmt::Display + std::ops::Neg
// {
//     term: Box<term::Term<T>>
// }

// impl<T> Sine<T> where T: Expression + Calculus + std::fmt::Display + std::ops::Neg {
//     pub fn new(term: Box<term::Term<T>>) -> Box<Sine<T>> {
//         Box::new(Sine { term })
//     }
// }

// impl<T> Expression for Sine<T> where T: Expression + Calculus + std::fmt::Display + std::ops::Neg {
//     fn evaluate(&self, x: f64) -> f64 {
//         x.sin()
//     }
// }

// impl<T> Calculus for Sine<T> where T: Expression + Calculus + std::fmt::Display + std::ops::Neg {
//     type DReturn = Cosine<T>;
//     type IReturn = Cosine<T>;

//     fn derivative(&self) -> Box<Cosine<T>> {
//         Cosine::new(self.term)
//     }

//     fn integral(&self) -> Box<Cosine<T>> {
//         Cosine::new(-*self.term)
//     }
// }

// pub struct Cosine<T> where T: Expression + Calculus + std::fmt::Display + std::ops::Neg
// {
//     term: Box<term::Term<T>>
// }

// impl<T> Cosine<T> where T: Expression + Calculus + std::fmt::Display + std::ops::Neg {
//     pub fn new(term: Box<term::Term<T>>) -> Box<Cosine<T>> {
//         Box::new(Cosine { term })
//     }
// }

// impl<T> Expression for Cosine<T> where T: Expression + Calculus + std::fmt::Display + std::ops::Neg {
//     fn evaluate(&self, x: f64) -> f64 {
//         x.sin()
//     }
// }

// impl<T> Calculus for Cosine<T> where T: Expression + Calculus + std::fmt::Display + std::ops::Neg {
//     type DReturn = Sine<T>;
//     type IReturn = Sine<T>;

//     fn derivative(&self) -> Box<Sine<T>> {
//         Sine::new(-*self.term)
//     }

//     fn integral(&self) -> Box<Sine<T>> {
//         Sine::new(Box::new(*self.term))
//     }
// }

// impl<T> std::ops::Neg for Sine<T> where T: Expression + Calculus + std::fmt::Display + std::ops::Neg {
//     type Output = Box<Sine<T>>;

//     fn neg(self) -> Self::Output {
//         Sine::new(
//             -*self.term
//         )
//     }
// }

/*

pub struct Tangent {
    term: term::Term
}

impl Tangent {
    pub fn new(term: term::Term) -> Box<Tangent> {
        Box::new(Tangent { term })
    }
}

pub struct Cotangent {
    term: term::Term
}

impl Cotangent {
    pub fn new(term: term::Term) -> Box<Cotangent> {
        Box::new(Cotangent { term })
    }
}

pub struct Secant {
    term: term::Term
}

impl Secant {
    pub fn new(term: term::Term) -> Box<Secant> {
        Box::new(Secant { term })
    }
}

pub struct Cosecant {
    term: term::Term
}

impl Cosecant {
    pub fn new(term: term::Term) -> Box<Cosecant> {
        Box::new(Cosecant { term })
    }
}
*/

pub enum VarFunc {
    sin(Box<dyn Maf>),
    cos(Box<dyn Maf>),
    tan(Box<dyn Maf>),
    cot(Box<dyn Maf>),
    sec(Box<dyn Maf>),
    csc(Box<dyn Maf>)
}

impl Maf for VarFunc {}

impl<T> Calculus for VarFunc<T> where T: Calculus + Maf {
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
