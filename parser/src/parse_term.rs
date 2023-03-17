use crate::ast::{BinOp, Term};
use crate::parse_factor::{match_factor, parse_factor};
use crate::parser::Parser;
use crate::token::Token;

pub(crate) fn match_term(parser: &mut Parser) -> bool {
  // if token is identifier, then next token can't be (
  match_factor(parser)
}

pub fn parse_term(parser: &mut Parser) -> Term {
  let token = parser.get_token();
  if !match_term(parser) {
    panic!("{:?} dont match factor", &token);
  }

  let factor = parse_factor(parser);
  let token = parser.get_token();
  if !(token.is_star() || token.is_slash()) {
    return Term::new(factor, None, None);
  }

  parser.advance_token();

  let op = if token.is_star() {
    BinOp::Time
  } else {
    BinOp::Div
  };

  Term::new(factor, Some(op), Some(Box::new(parse_term(parser))))
}
