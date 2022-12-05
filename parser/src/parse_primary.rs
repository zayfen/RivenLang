use crate::ast::{Identifier, Kind, Primary, PrimaryValue};
use crate::parse_literal::parse_literal;
use crate::parser::Parser;
use crate::token::{NumberType, Token};

pub(crate) fn parse_primary(parser: &mut Parser, token: Token) -> Result<Primary, String> {
  let ref_token = &token;

  let primary_value = match ref_token {
    Token::Id { name } => PrimaryValue::Variable(Identifier {
      kind: Kind::Identifier,
      name: name.to_owned(),
    }),
    Token::Number {
      number_type,
      int,
      float,
    }
    | Token::String => PrimaryValue::Constant(parse_literal(parser, token).unwrap()),
  };

  Ok(Primary::new(primary_value))
  // if let Token::String { value } = token.clone() {
  //   return Ok(Primary::new(PrimaryValue::Constant(
  //     parse_literal(parser, token).unwrap(),
  //   )));
  // }

  // if let Token::Number {
  //   number_type,
  //   int,
  //   float,
  // } = token.clone()
  // {
  //   return Ok(Primary::new(PrimaryValue::Constant(
  //     parse_literal(parser, token).unwrap(),
  //   )));
  // }

  // Err(format!("parse_primary error: unrecognized token {}", token))
}
