use crate::ast::Identifier;
use crate::parser::Parser;
use crate::token::{NumberType, Token};

pub(crate) fn parse_primary(parser: &mut Parser, token: Token) -> Result<Literal, String> {
  match token {
    Token::Id { name } => ,
    Token::String { value } => Ok(Literal::string(value)),
    _ => Err(format!("Parse term error with token: {}", token)),
  }
}
