//! 解析expression
use crate::ast::{Kind, Expression, AssignmentExpression, AssignmentOperator};
use crate::parser::Parser;
use crate::token::Token;

pub fn parse_expr (parser: &mut Parser) -> Result<Expression, String> {
  let tokens = parser.lookahead2();
  match tokens.0 {
    Token::Id { name } => {
      if tokens.1 == Token::Equal {
        Ok(parse_assign_expr(&mut parser, name).unwrap())
      }
    }
  }

  Err("parse_expr unknown error".to_owned())
}

fn parse_assign_expr (parser: &mut Parser, id: String) -> Result<Expression::AssignmentExpression, String> {
  return Ok(Expression::AssignmentExpression(Box::new(AssignmentExpression {
    kind: Kind::AssignmentExpression,
    operator: Some(AssignmentOperator::new(EnumAssignmentOperators::Assign))
  })))
}
