#![allow(warnings)]

use crate::row::{Datum, Row};
use core::panic;
use std::fmt;
use std::ops;
use Expr::*;

#[derive(Debug)]
pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for ArithOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            ArithOp::Add => '+',
            ArithOp::Sub => '-',
            ArithOp::Mul => '*',
            ArithOp::Div => '/',
        };
        write!(f, "{}", display_str)
    }
}

#[derive(Debug)]
pub enum LogOp {
    And,
    Or,
    Not,
}

impl fmt::Display for LogOp {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            LogOp::And => "&&",
            LogOp::Or => "||",
            LogOp::Not => "!",
        };
        write!(f, "{}", display_str)
    }
}

#[derive(Debug)]
pub enum RelOp {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
}

impl fmt::Display for RelOp {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            RelOp::Eq => "==",
            RelOp::Ne => "!=",
            RelOp::Gt => ">",
            RelOp::Ge => ">=",
            RelOp::Lt => "<",
            RelOp::Le => "<=",
        };
        write!(f, "{}", display_str)
    }
}

/***************************************************************************************************/
#[derive(Debug)]
pub enum Expr {
    CID(usize),
    Literal(Datum),
    ArithExpr(Box<Expr>, ArithOp, Box<Expr>),
    RelExpr(Box<Expr>, RelOp, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CID(cid) => write!(f, "${}", cid),
            Literal(v) => write!(f, "{}", v),
            ArithExpr(lhs, op, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            RelExpr(lhs, op, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
        }
    }
}

/***************************************************************************************************/
impl ops::Add for Expr {
    type Output = Self;

    fn add(self, other: Self) -> Expr {
        Expr::ArithExpr(Box::new(self), ArithOp::Add, Box::new(other))
    }
}

impl ops::Sub for Expr {
    type Output = Self;

    fn sub(self, other: Self) -> Expr {
        Expr::ArithExpr(Box::new(self), ArithOp::Sub, Box::new(other))
    }
}

impl ops::Mul for Expr {
    type Output = Self;

    fn mul(self, other: Self) -> Expr {
        Expr::ArithExpr(Box::new(self), ArithOp::Mul, Box::new(other))
    }
}

impl ops::Div for Expr {
    type Output = Self;

    fn div(self, other: Self) -> Expr {
        Expr::ArithExpr(Box::new(self), ArithOp::Div, Box::new(other))
    }
}

impl Expr {
    pub fn eval<'a>(&'a self, row: &'a Row) -> Datum {
        match self {
            CID(cid) => row.get_column(*cid).clone(),
            Literal(lit) => lit.clone(),
            ArithExpr(b1, op, b2) => {
                let b1 = b1.eval(row);
                let b2 = b2.eval(row);
                let res = match (b1, op, b2) {
                    (Datum::INT(i1), ArithOp::Add, Datum::INT(i2)) => i1 + i2,
                    (Datum::INT(i1), ArithOp::Sub, Datum::INT(i2)) => i1 - i2,
                    (Datum::INT(i1), ArithOp::Mul, Datum::INT(i2)) => i1 * i2,
                    (Datum::INT(i1), ArithOp::Div, Datum::INT(i2)) => i1 / i2,
                    _ => panic!("Internal error: Operands of ArithOp not resolved yet."),
                };
                Datum::INT(res)
            }
            RelExpr(b1, op, b2) => {
                let b1 = b1.eval(row);
                let b2 = b2.eval(row);
                let res = match (b1, op, b2) {
                    (Datum::INT(i1), RelOp::Eq, Datum::INT(i2)) => i1 == i2,
                    (Datum::INT(i1), RelOp::Ne, Datum::INT(i2)) => i1 != i2,
                    (Datum::INT(i1), RelOp::Le, Datum::INT(i2)) => i1 <= i2,
                    (Datum::INT(i1), RelOp::Lt, Datum::INT(i2)) => i1 < i2,
                    (Datum::INT(i1), RelOp::Ge, Datum::INT(i2)) => i1 >= i2,
                    (Datum::INT(i1), RelOp::Gt, Datum::INT(i2)) => i1 > i2,
                    _ => panic!("Internal error: Operands of RelOp not resolved yet."),
                };
                Datum::BOOL(res)
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let e: Expr = RelExpr(
            Box::new(CID(0) + CID(1)),
            RelOp::Gt,
            Box::new(Literal(Datum::INT(30))),
        );
        println!("{}", e)
    }
}
