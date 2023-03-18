use crate::ast::{Expression, ExpressionValue};
use crate::parse_component_arithmetic_expr::{
  match_component_arith_expr, parse_component_arithmetic_expr,
};
use crate::parser::Parser;

pub fn match_expression(parser: &mut Parser) -> bool {
  match_component_arith_expr(parser)
}

pub fn parse_expression(parser: &mut Parser) -> Expression {
  if !match_component_arith_expr(parser) {
    panic!(
      "parse_expression error, unexpected token({:?})",
      parser.get_token()
    );
  }
  Expression::from(ExpressionValue::ComponentArithmeticExpr(
    parse_component_arithmetic_expr(parser),
  ))
}
