
use crate::lexer::Token;
use Token::Operator;

pub struct UnknownOp {}

#[derive(Debug, Clone, Copy)]
pub enum PrefixOp {
    Plus, Negative, Not, BitwiseNot,
}

impl TryFrom<Token> for PrefixOp {
    type Error = UnknownOp;

    fn try_from(c: Token) -> Result<Self, Self::Error> {
        use PrefixOp::*;
        match c {
            Operator('+') => Ok(Plus),
            Operator('-') => Ok(Negative),
            Operator('!') => Ok(Not),
            Operator('~') => Ok(BitwiseNot),
            _ => Err(UnknownOp {})
        }
    }
}

impl PrefixOp {
    pub fn precedence(self) -> u32 {
        use PrefixOp::*;
        match self {
            Plus | Negative | Not | BitwiseNot => 10
        }
    }

    pub fn operate_on(self, value: i64) -> i64 {
        use PrefixOp::*;
        match self {
            Plus => value,
            Negative => -value,
            Not => (value == 0) as i64,
            BitwiseNot => !value
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub enum InfixOp {
    Multiply, Divide, Modulo,
    Add, Subtract,
    Less, Greater,
    Equal, NotEqual,
    And, Or
}

impl TryFrom<Token> for InfixOp {
    type Error = UnknownOp;

    fn try_from(c: Token) -> Result<Self, Self::Error> {
        use InfixOp::*;
        match c {
            Operator('+') => Ok(Add),
            Operator('-') => Ok(Subtract),
            Operator('*') => Ok(Multiply),
            Operator('/') => Ok(Divide),
            Operator('%') => Ok(Modulo),
            Operator('<') => Ok(Less),
            Operator('>') => Ok(Greater),
            Operator('=') => Ok(Equal),
            Operator('~') => Ok(NotEqual),
            Operator('&') => Ok(And),
            Operator('|') => Ok(Or),
            _ => Err(UnknownOp {})
        }
    }
}

impl InfixOp {
    pub fn precedence(self) -> u32 {
        use InfixOp::*;
        match self {
            Multiply | Divide | Modulo => 7,
            Add | Subtract => 6,
            Less | Greater => 5,
            Equal | NotEqual => 4,
            And | Or => 3
        }
    }

    pub fn is_left_associative(self) -> bool {
        match self {
            _ => true // All our infix operators are left associative
        }
    }

    pub fn operate_on(self, lhs: i64, rhs:i64) -> i64 {
        use InfixOp::*;
        match self {
            Multiply => lhs * rhs,
            Divide => lhs / rhs,
            Modulo => lhs % rhs,
            Add => lhs + rhs,
            Subtract => lhs - rhs,
            Less => (lhs < rhs) as i64,
            Greater => (lhs > rhs) as i64,
            Equal => (lhs == rhs) as i64,
            NotEqual => (lhs != rhs) as i64,
            And => lhs & rhs,
            Or => lhs | rhs
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PostfixOp {
    Factorial
}

impl TryFrom<Token> for PostfixOp {
    type Error = UnknownOp;

    fn try_from(c: Token) -> Result<Self, Self::Error> {
        use PostfixOp::*;
        match c {
            Operator('!') => Ok(Factorial),
            _ => Err(UnknownOp {})
        }
    }
}

impl PostfixOp {
    pub fn precedence(self) -> u32 {
        use PostfixOp::*;
        match self {
            Factorial => 11
        }
    }

    pub fn operate_on(self, value: i64) -> i64 {
        use PostfixOp::*;
        match self {
            Factorial => todo!()
        }
    }
}
