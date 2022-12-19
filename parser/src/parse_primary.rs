pub(crate) use crate::ast::{Identifier, Kind, Primary, PrimaryValue};
use crate::parse_literal::parse_literal;
use crate::parser::Parser;
use crate::token::Token;

pub fn parse_primary(parser: &mut Parser, token: Token) -> Result<Primary, String> {
  let ref_token = &token;

  let primary_value = match ref_token {
    Token::Id { name } => Ok(PrimaryValue::Variable(Identifier(name.to_owned()))),
    Token::Number {
      number_type: _,
      int: _,
      float: _,
    } => Ok(PrimaryValue::Constant(
      parse_literal(parser, token).unwrap(),
    )),
    Token::String { value: _ } => Ok(PrimaryValue::Constant(
      parse_literal(parser, token).unwrap(),
    )),
    _ => Err("Unexcepted token when parse_primary".to_owned()),
  };

  Ok(Primary::new(primary_value.unwrap()))
}
