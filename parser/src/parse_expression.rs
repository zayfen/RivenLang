//! parse expressions
//! include arithmetic expression, string expression and so on
use crate::ast::{ArithmeticExpr, EnumBinaryOperators};
use crate::parse_term::parse_term;
use crate::parser::Parser;
use crate::token::Token;

pub fn parse_arithmetic_expr(parser: &mut Parser) -> Result<ArithmeticExpr, String> {
  let term_result = parse_term(parser);

  let curr_token = parser.get_token();

  let result = match curr_token {
    Token::Plus | Token::Minus => {
      let left = term_result.unwrap();
      let operator = if curr_token == Token::Plus {
        Some(EnumBinaryOperators::Add)
      } else {
        Some(EnumBinaryOperators::Minus)
      };

      // advance cursor
      parser.next_token();
      let right = parse_arithmetic_expr(parser);

      ArithmeticExpr::new(
        left,
        operator,
        if let Ok(expr) = right {
          Some(Box::new(expr))
        } else {
          None
        },
      )
    }
    _ => ArithmeticExpr::new(term_result.unwrap(), None, None),
  };

  Ok(result)
}
