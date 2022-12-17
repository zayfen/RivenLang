pub(crate) use crate::ast::{Identifier, Kind, Primary, PrimaryValue};
use crate::parse_literal::parse_literal;
use crate::parser::Parser;
use crate::token::Token;

pub(crate) fn parse_primary(parser: &mut Parser, token: Token) -> Result<Primary, String> {
  let ref_token = &token;

  let primary_value = match ref_token {
    Token::Id { name } => Ok(PrimaryValue::Variable(Identifier {
      kind: Kind::Identifier,
      name: name.to_owned(),
    })),
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

#[cfg(test)]
mod tests {
  use super::parse_primary;
  use crate::{
    ast::{Literal, Primary, PrimaryValue},
    parser::Parser,
  };

  #[test]
  fn test_parse_primary_string() {
    let mut parser = Parser::new("'123'");
    let token = parser.next_token();
    let primary_node_result = parse_primary(&mut parser, token);
    let primary_node = primary_node_result.unwrap();

    assert_eq!(
      primary_node,
      Primary::new(PrimaryValue::Constant(Literal::string("123".to_owned())))
    );
  }

  #[test]
  fn test_parse_primary_number() {
    let mut parser = Parser::new("123");
    let token = parser.next_token();
    let primary_node_result = parse_primary(&mut parser, token);
    let primary_node = primary_node_result.unwrap();

    assert_eq!(
      primary_node,
      Primary::new(PrimaryValue::Constant(Literal::number(123f64)))
    );
  }
}
