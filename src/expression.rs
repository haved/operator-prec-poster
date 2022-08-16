
use crate::operator::{PrefixOp, InfixOp, PostfixOp};

#[derive(Debug)]
pub enum Expression {
    Number(i64),
    PrefixOp(PrefixOp, Box<Expression>),
    InfixOp(Box<Expression>, InfixOp, Box<Expression>),
    PostfixOp(Box<Expression>, PostfixOp)
}

impl Expression {
    pub fn evaluate(&self) -> i64 {
        use Expression::*;
        match self {
            Number(value) => *value,
            PrefixOp(op, rhs) => op.operate_on(rhs.evaluate()),
            InfixOp(lhs, op, rhs) => op.operate_on(lhs.evaluate(), rhs.evaluate()),
            PostfixOp(lhs, op) => op.operate_on(lhs.evaluate())
        }
    }
}
