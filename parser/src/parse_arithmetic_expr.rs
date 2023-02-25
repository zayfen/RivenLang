use crate::ast::{ArithmeticExpr, BinOp};
use crate::parse_term::{match_term, parse_term};
use crate::parser::Parser;
use crate::token::Token;

pub(crate) fn match_arithmetic_expr(token: Token, next_token: Token) -> bool {
  match_term(token)
}

pub fn parse_arithmetic_expr(parser: &mut Parser) -> ArithmeticExpr {
  let token = parser.get_token();
  let next_token = parser.peek_token();

  if !match_arithmetic_expr(token.clone(), next_token) {
    panic!("{:?} dont match term", &token);
  }

  let term = parse_term(parser);
  let token = parser.get_token();

  if !(token.is_plus() || token.is_minus()) {
    return ArithmeticExpr::new(term, None, None);
  }

  parser.advance_token();

  let op = if token.is_plus() {
    BinOp::Add
  } else {
    BinOp::Min
  };

  ArithmeticExpr::new(
    term,
    Some(op),
    Some(Box::new(parse_arithmetic_expr(parser))),
  )
}
