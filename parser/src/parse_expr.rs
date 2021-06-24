//! 解析expression
use crate::ast::{AssignmentExpression, AssignmentOperator, Expression, Kind};
use crate::parser::Parser;
use crate::token::Token;

pub fn parse_expr(parser: &mut Parser) -> Result<Expression, String> {}
