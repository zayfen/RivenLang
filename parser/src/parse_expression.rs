use crate::ast::{Expression, ExpressionValue};
use crate::parse_arithmetic_expr::{match_arithmetic_expr, parse_arithmetic_expr};
use crate::parse_call_expr::{match_call_expr, parse_call_expr};
use crate::parser::Parser;

pub fn parse_expression(parser: &mut Parser) -> Expression {
  let token = parser.get_token();
  let next_token = parser.peek_token();

  if match_call_expr(token.clone(), next_token) {
    return Expression::from(ExpressionValue::CallExpr(parse_call_expr(parser)));
  } else if match_arithmetic_expr(token) {
    return Expression::from(ExpressionValue::ArithmeticExpr(parse_arithmetic_expr(
      parser,
    )));
  }

  panic!(
    "parse_expression error, token({:?}) not match",
    parser.get_token()
  );
}
