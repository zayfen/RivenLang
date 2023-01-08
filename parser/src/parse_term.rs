use crate::ast::{EnumBinaryOperators, Term};
use crate::parse_primary::parse_primary;
use crate::parser::Parser;
use crate::token::Token;

pub fn parse_term(parser: &mut Parser) -> Result<Term, String> {
  let primary = parse_primary(parser);
  if primary.is_err() {
    panic!("expect primary,but found {:?}", primary)
  }

  let op_token = parser.get_token();
  let mut op: Option<EnumBinaryOperators> = None;

  op = match op_token {
    Token::Star => Some(EnumBinaryOperators::Time),
    Token::Slash => Some(EnumBinaryOperators::Div),
    Token::Percent => Some(EnumBinaryOperators::Mode),
    _ => None,
  };

  if op.is_some() {
    parser.next_token();
    let right_term = parse_term(parser);
    Ok(Term::new(
      primary.unwrap(),
      op,
      Some(Box::new(right_term.unwrap())),
    ))
  } else {
    Ok(Term::new(primary.unwrap(), None, None))
  }
}
