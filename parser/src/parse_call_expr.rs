use crate::ast::{CallExpr, Identifier};
use crate::parser::Parser;
use crate::token::Token;

pub(crate) fn match_call_expr(token: Token, next_token: Token) -> bool {
  token.is_id() && next_token.is_lpar()
}

pub fn parse_call_expr(parser: &mut Parser) -> CallExpr {
  let token = parser.get_token();
  let next_token = parser.peek_token();

  if !match_call_expr(token.clone(), next_token) {
    panic!("{:?} dont match call expr", &token);
  }

  let fn_name = match token {
    Token::Id { name } => name,
    _ => panic!("parse_call_expr error, should be Token::Id"),
  };

  parser.advance_token();
  parser.advance_token();

  let mut args: Vec<Identifier> = vec![];

  let mut token = parser.get_token();
  while token.is_id() {
    if let Token::Id { name } = token {
      args.push(Identifier::from(name.as_str()));
    }
    // ,
    token = parser.advance_token();
    if token.is_comma() {
      token = parser.advance_token();
    }
  }

  // should be right parent
  parser.eat_token(Token::RPar);

  CallExpr::new(Identifier::from(fn_name.as_str()), args)
}
