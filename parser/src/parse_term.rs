//! parse_term.rs
//! 解析term, 这里就是构造数字和字符串
use crate::ast::{Kind, Literal, LiteralValue};
use crate::parser::Parser;
use crate::token::{NumberType, Token, BigIntHelper};

pub fn parse_term(parser: &mut Parser) -> Result<Literal, String> {
  let token = parser.next_token();
  match token {
    Token::Number {
      number_type,
      int,
      float,
    } => match number_type {
      NumberType::Float => Ok(Literal::new(LiteralValue::Float(float))),
      NumberType::Int => Ok(Literal::new(LiteralValue::Integer(int.to_i64()))),
      _ => panic!("parse_term: don't support NumberType::Complex"),
    },
    Token::String { value } => Ok(Literal::new(LiteralValue::Str(value))),
    _ => panic!("Parse term error")
  }
}
