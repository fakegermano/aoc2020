use std::fmt::{Debug, Error, Formatter};

pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>)
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Add,
    Mul
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Add => write!(fmt, "+"),
        }
    }
}