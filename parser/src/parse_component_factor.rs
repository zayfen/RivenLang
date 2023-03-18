use crate::{parser::Parser, ast::{ComponentFactor, Identifier, ComponentFactorValue}, parse_arithmetic_expr::{match_arithmetic_expr, parse_arithmetic_expr}, token::Token};


pub fn match_component_factor(parser: &mut Parser) -> bool {
  parser.get_token().is_lpar() || match_arithmetic_expr(parser)
}

pub fn parse_component_factor(parser: &mut Parser) -> ComponentFactor {
  if !match_component_factor(parser) {
    panic!("parse component factor error, unexpected token: {:?}", parser.get_token())
  }

  // lick ( 1 + 2) * 3
  if parser.get_token().is_lpar() {
    parser.eat_token(Token::LPar);
    let component_factor = parse_component_factor(parser);
    parser.eat_token(Token::RPar);
    return ComponentFactor::from(ComponentFactorValue::ComponentFactor(Some(Box::new(component_factor))));
  }

  let arith_expr = parse_arithmetic_expr(parser);
  ComponentFactor::from(ComponentFactorValue::ArithmeticExpr(arith_expr))
}


#[test]
fn test_parse_component_factor() {
  let mut parser = Parser::new("(1+2)*3");
  let expr = parse_component_factor(&mut parser);
  assert_eq!(expr.0, true);
  assert_eq!(parser.get_token(), Token::Star);
}
