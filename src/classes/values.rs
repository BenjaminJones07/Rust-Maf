use super::{Calculus, Expression, Maf, Term};

macro_rules! enumy {
    (pub enum $name:ident {
        $($variant:ident),*,
    }) => {
        #[allow(dead_code)]
        #[derive(Debug)]
        pub enum $name {
            $($variant),*
        }

        impl $name {
            #[allow(dead_code)]
            pub fn from(name: String) -> Self {
                match name.to_uppercase().as_str() {
                    $(stringify!($variant) => $name::$variant),*,
                    &_ => panic!("Unknown enum variant: {}", name),
                }
            }

            pub fn name(&self) -> String {
                match self {
                    $($name::$variant => stringify!($variant).to_lowercase()),*
                }
            }

            #[allow(dead_code)]
            pub fn names() -> Vec<String> {
                vec![
                    $(stringify!($variant).to_lowercase()),*
                ]
            }

            pub fn cloned(&self) -> Self {
                match self {
                    $($name::$variant => $name::$variant),*
                }
            }
        }
    };
}

enumy! {
    pub enum Vars {
        X,
        Y,
        Z,
    }
}

enumy! {
    pub enum Consts {
        E,
        I,
        PI,
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Values {
    NAN,
    Variable(Vars),
    Constant(Consts),
}

impl Maf for Values {
    fn neg(&self) -> Box<dyn Maf> {
        match self {
            Values::NAN => Box::new(Values::NAN),
            Values::Variable(v) => {
                Term::new(-1f64, vec![Box::new(Values::Variable(v.cloned()))], 1f64)
            }
            Values::Constant(c) => {
                Term::new(-1f64, vec![Box::new(Values::Constant(c.cloned()))], 1f64)
            }
        }
    }

    fn cloned(&self) -> Box<dyn Maf> {
        match self {
            Values::NAN => Box::new(Values::NAN),
            Values::Variable(v) => Box::new(Values::Variable(v.cloned())),
            Values::Constant(c) => Box::new(Values::Constant(c.cloned())),
        }
    }

    fn reciprical(&self) -> Box<dyn Maf> {
        match self {
            Values::NAN => Box::new(Values::NAN),
            Values::Variable(v) => {
                Term::new(1f64, vec![Box::new(Values::Variable(v.cloned()))], -1f64)
            }
            Values::Constant(c) => {
                Term::new(1f64, vec![Box::new(Values::Constant(c.cloned()))], -1f64)
            }
        }
    }
}

impl Calculus for Values {
    fn derivative(&self) -> Box<dyn Maf> {
        match self {
            Values::NAN => Box::new(Values::NAN),
            Values::Variable(v) => Term::new(
                1f64,
                vec![Term::new(
                    1f64,
                    vec![Box::new(Values::Variable(v.cloned()))],
                    1f64,
                )],
                1f64,
            ),
            Values::Constant(c) => Term::new(
                0f64,
                vec![Term::new(
                    0f64,
                    vec![Box::new(Values::Constant(c.cloned()))],
                    1f64,
                )],
                1f64,
            ),
        }
    }

    fn integral(&self) -> Box<dyn Maf> {
        match self {
            Values::NAN => Box::new(Values::NAN),
            Values::Variable(v) => Term::new(
                1f64,
                vec![Term::new(
                    1f64,
                    vec![Box::new(Values::Variable(v.cloned()))],
                    1f64,
                )],
                1f64,
            ),
            Values::Constant(c) => Term::new(
                0f64,
                vec![Term::new(
                    0f64,
                    vec![Box::new(Values::Constant(c.cloned()))],
                    1f64,
                )],
                1f64,
            ),
        }
    }
}

impl Expression for Values {
    fn evaluate(&self, x: f64, y: f64, z: f64) -> f64 {
        match self {
            Values::NAN => 0f64,
            Values::Variable(v) => match v {
                Vars::X => x,
                Vars::Y => y,
                Vars::Z => z,
            },
            Values::Constant(c) => match c {
                Consts::E => 2.718281828459045,
                Consts::I => 1i64 as f64,
                Consts::PI => 3.141592653589793,
            },
        }
    }
}

impl std::fmt::Display for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Values::NAN => write!(f, "NaN"),
            Values::Variable(v) => write!(f, "{}", v.name()),
            Values::Constant(c) => write!(f, "{}", c.name()),
        }
    }
}
