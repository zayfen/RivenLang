//! parse_term.rs
//! 解析term, 这里就是构造数字和字符串
use crate::parser::Parser;
use crate::ast::Literal;
use crate::token::{Token, NumberType};

pub fn parse_term(parser: &mut Parser) -> Result<Literal, String> {
  let token = parser.next();
  match token {
    Token::Number {number_type, int, float} => {
      match number_type {
        NumberType::Float => {
          Ok(Literal::number(float))
        },
        _ => Err("Parse term error".to_owned())
      }
    },
    _ => Err("Parse term error".to_owned())
  }
}
