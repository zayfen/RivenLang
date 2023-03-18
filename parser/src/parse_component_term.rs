use crate::ast::{BinOp, ComponentTerm};
use crate::parse_component_factor::{match_component_factor, parse_component_factor};
use crate::parser::Parser;
use crate::token::Token;

pub(crate) fn match_component_term(parser: &mut Parser) -> bool {
  match_component_factor(parser)
}

pub fn parse_component_term(parser: &mut Parser) -> ComponentTerm {
  let token = parser.get_token();
  if !match_component_term(parser) {
    panic!("parse component term error: unexpected token {:?}", &token);
  }

  let factor = parse_component_factor(parser);
  let token = parser.get_token();
  if !(token.is_star() || token.is_slash()) {
    return ComponentTerm::new(factor, None, None);
  }

  // skip BinOp::Time or BinOp::Div
  parser.advance_token();

  let op = if token.is_star() {
    BinOp::Time
  } else {
    BinOp::Div
  };

  ComponentTerm::new(factor, Some(op), Some(Box::new(parse_component_term(parser))))
}


#[test]
pub fn test_parse_component_term() {
  let mut parser = Parser::new("(1+2)*3 name");
  let expr = parse_component_term(&mut parser);
  assert_eq!(expr.1, Some(BinOp::Time));
  assert_eq!(parser.get_token(), Token::Id{ name: "name".to_string()});
}
