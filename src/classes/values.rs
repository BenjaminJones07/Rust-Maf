pub enum Vars {
    X,
    Y,
    Z
}

impl 

pub enum Consts {
    E,
    I,
    Pi
}

pub enum Values {
    NAN,
    Variable(Vars),
    Constant(Consts)
}