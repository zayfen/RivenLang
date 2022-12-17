//! parse_literal.rs
//! 解析term, 这里就是构造数字和字符串
use crate::ast::Literal;
use crate::parser::Parser;
use crate::token::{NumberType, Token};

pub fn parse_literal(_parser: &mut Parser, token: Token) -> Result<Literal, String> {
  match token {
    Token::Number {
      number_type,
      int,
      float,
    } => match number_type {
      NumberType::Float => Ok(Literal::number(float)),
      NumberType::Int => Ok(Literal::number(int.to_u32_digits().1[0].into())),
      _ => Err("parse_term: don't support NumberType::Complex".to_owned()),
    },
    Token::String { value } => Ok(Literal::string(value)),
    _ => Err(format!("Parse term error with token: {}", token)),
  }
}
