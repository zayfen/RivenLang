use crate::ast::{Identifier, Kind, Primary, PrimaryValue};
use crate::parse_literal::parse_literal;
use crate::parser::Parser;
use crate::token::{NumberType, Token};

pub(crate) fn parse_primary(parser: &mut Parser, token: Token) -> Result<Primary, String> {
  let ref_token = &token;

  let primary_value = match ref_token {
    Token::Id { name } => Ok(PrimaryValue::Variable(Identifier {
      kind: Kind::Identifier,
      name: name.to_owned(),
    })),
    Token::Number {
      number_type,
      int,
      float,
    } => Ok(PrimaryValue::Constant(
      parse_literal(parser, token).unwrap(),
    )),
    Token::String { value } => Ok(PrimaryValue::Constant(
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
    ast::{EnumLiteral, Kind, Literal, Primary, PrimaryValue},
    parser::Parser,
  };

  #[test]
  fn test_parse_primary_string() {
    let mut parser = Parser::new("'123'");
    let token = parser.next_token();
    let primary_node_result = parse_primary(&mut parser, token);
    let primary_node = primary_node_result.unwrap();

    if let Primary {
      kind,
      value: PrimaryValue::Constant(primary_value),
    } = primary_node
    {
      if let Literal { kind, value } = primary_value {
        assert_eq!(kind, Kind::Literal);
        assert_eq!(value, EnumLiteral::Number("123"));
      }
    }
  }

  #[test]
  fn test_parse_primary_number() {
    let mut parser = Parser::new(123);
    let token = parser.next_token();
    let primary_node_result = parse_primary(&mut parser, token);
    let primary_node = primary_node_result.unwrap();

    if let Primary {
      kind,
      value: PrimaryValue::Constant(primary_value),
    } = primary_node
    {
      if let Literal { kind, value } = primary_value {
        assert_eq!(kind, Kind::Literal);
        assert_eq!(value, EnumLiteral::Number(123f64));
      }
    }
  }
}
