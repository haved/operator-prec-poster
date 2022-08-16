use std::io::{self, Write};

mod lexer;
mod operator;
mod expression;
mod parse_result;

use lexer::{Lexer, Token};
use expression::Expression;
use operator::{PrefixOp, InfixOp, PostfixOp};
use parse_result::{ParseResult, ParseError};

/// Consumes expressions and operators to build an expression tree.
/// Call with min_precedence 0 to parse the entire expression.
/// Recurses with higher min_precedence to parse sub-expressions.
///
/// Example: When parsing the RHS of "4 + ___"
/// The * operator has high enough precedence that 5*6 is a valid RHS.
/// The - operator does not, so 5-6 will not be a valid RHS.
/// Instead, 5 is returned up the stack to make 4+5 before - can be consumed.
fn parse_expression(lexer: &mut Lexer, min_precedence: u32) -> ParseResult<Expression> {
    // If we have a prefix operator, always consume it, and recurse to parse its operand.
    // Otherwise, parse an atomic expression, e.g. 5 or (2+2) in parenthesies
    let mut expression = if let Ok(op) = PrefixOp::try_from(lexer.current()) {
        lexer.advance(); // Consume the prefix operator token
        let rhs = parse_expression(lexer, op.precedence())?;
        Expression::PrefixOp(op, Box::new(rhs))
    } else {
        parse_atomic_expression(lexer)?
    };

    // Loop to consume all infix and postfix operators where precedence >= min_precedence.
    // Otherwise return, to let the operator be consumed higher up in the expression tree.
    loop {
        if let Ok(op) = InfixOp::try_from(lexer.current()) {
            if op.precedence() < min_precedence { break; }
            lexer.advance(); // Consume the infix operator token
            // We recurse to parse its RHS, taking min_precedence from the operator.
            // If we are left associative, make min_precedence one higher.
            let rhs = parse_expression(lexer, op.precedence() + op.is_left_associative() as u32)?;
            expression = Expression::InfixOp(Box::new(expression), op, Box::new(rhs));
        }
        else if let Ok(op) = PostfixOp::try_from(lexer.current()) {
            if op.precedence() < min_precedence { break; }
            lexer.advance(); // Consume the postfix operator token
            expression = Expression::PostfixOp(Box::new(expression), op);
        }
        else { break; }
    }

    Ok(expression)
}

fn parse_atomic_expression(lexer: &mut Lexer) -> ParseResult<Expression> {
    match lexer.take() {
        Token::Number(value) => Ok(Expression::Number(value)),
        Token::LeftParen => {
            let expression = parse_expression(lexer, 0)?;
            lexer.consume(Token::RightParen, ")")?;
            Ok(expression)
        },
        _ => Err(ParseError::expected("expression"))
    }
}

fn main() -> io::Result<()> {
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        if buffer.trim().len() == 0 { break; }

        let mut lexer = Lexer::new(&mut buffer);
        match parse_expression(&mut lexer, 0) {
            Ok(ast) => {
                println!("{0}", ast.evaluate());
            },
            Err(err) => {
                eprintln!("{0}", err.got(lexer.current()));
                break;
            },
        };
    }
    Ok(())
}
