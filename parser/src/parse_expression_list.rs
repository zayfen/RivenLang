use crate::{parser::Parser, ast::{ExpressionList, Expression, Identifier}, parse_expression::{match_expression, parse_expression}, token::Token};


pub fn parse_expression_list(parser: &mut Parser) -> ExpressionList {
  if !match_expression(parser) {
    return ExpressionList(vec![]);
  }

  let mut expr_list: Vec<Expression> = vec![];

  let expr = parse_expression(parser);
  expr_list.push(expr);
  while parser.get_token().is_comma() {
    parser.eat_token(Token::Comma);
    let expr = parse_expression(parser);
    expr_list.push(expr);
  }

  ExpressionList(expr_list)
}


#[test]
pub fn test_parse_expr_list() {
  let code = "\"name\", 1+2, 3*4+name, call(name, 1+2)";
  let mut parser = Parser::new(code);
  let expr_list = parse_expression_list(&mut parser);
  dbg!(&expr_list);
  assert_eq!(expr_list.0.len(), 3);
}
