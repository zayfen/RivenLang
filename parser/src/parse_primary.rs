use num_traits::ToPrimitive;

use crate::ast::{Primary, PrimaryValue};
use crate::parser::Parser;
use crate::token::Token;

pub(crate) fn match_primary(token: Token) -> bool {
  token.is_number() || token.is_string()
}

pub fn parse_primary(parser: &mut Parser) -> Primary {
  let token = parser.get_token();
  let ref_token = &token;

  let value: PrimaryValue = match ref_token {
    Token::Number {
      number_type,
      int,
      float,
    } => {
      if number_type.is_int() {
        PrimaryValue::Number(int.to_f64().unwrap())
      } else {
        PrimaryValue::Number(*float)
      }
    }

    Token::String { value } => PrimaryValue::String(value.to_string()),
    _ => panic!("Mismatched token {:?}", *ref_token),
  };

  // advance token
  parser.advance_token();

  Primary::from(value)
}
