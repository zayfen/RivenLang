use crate::{ast::{ComponentArithmeticExpr, BinOp}, parse_component_term::{match_component_term, parse_component_term}, parser::Parser};
use crate::token::Token;

pub fn match_component_arith_expr (parser: &mut Parser) -> bool {
  match_component_term(parser)
}

pub fn parse_component_arithmetic_expr(parser: &mut Parser) -> ComponentArithmeticExpr {
  if !match_component_arith_expr(parser) {
    panic!("parse component arithmetic expr error: unexpected token {:?}", parser.get_token());
  }

  let term = parse_component_term(parser);
  let token = parser.get_token();

  if !(token.is_plus() || token.is_minus()) {
    return ComponentArithmeticExpr::new(term, None, None);
  }

  parser.advance_token();

  let op = BinOp::from(token);

  ComponentArithmeticExpr::new(
    term,
    Some(op),
    Some(Box::new(parse_component_arithmetic_expr(parser))),
  )
}


#[test]
fn test_parse_component_arithmetic_expr() {
  let mut parser = Parser::new("(1 - (1+2)*3) name");
  let expr = parse_component_arithmetic_expr(&mut parser);
  assert_eq!(expr.1, None);
  assert_eq!(parser.get_token(), Token::Id{ name: "name".to_owned() });
}
