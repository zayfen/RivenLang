///! 解析expression
use crate::ast::Expression;
use crate::parser::Parser;
use crate::token::Token;

pub fn parse_expr(parser: &mut Parser) -> Result<Expression, String> {
  let token = parser.next_token()();
  match token {
    Token::Id { name } => {
      if parser.lookahead == Token::Equal {
        parser.next();
        Ok(parse_assign_expr(&mut parser, name).unwrap())
      }
    }
  }

  Err("parse_expr unknown error".to_owned())
}

fn parse_assign_expr(
  parser: &mut Parser,
  id: String,
) -> Result<Expression::AssignmentExpression, String> {
  return Ok(Expression::AssignmentExpression {});
}
