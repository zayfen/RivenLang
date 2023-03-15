use crate::{ast::{CompareExpr, CompareOp, LogicExpr, LogicOp, Expression, ExpressionValue}, parser::Parser, parse_expression::{match_expression, parse_expression}, token::Token};


pub fn match_compare_expr(parser: &mut Parser) -> bool {
  match_expression(parser)
}


pub fn parse_compare_expr(parser: &mut Parser) -> CompareExpr {
  if !match_compare_expr(parser) {
    panic!("parse compare expr error, leading token not matched: {}", parser.get_token());
  }  

  let left_expr = parse_expression(parser);
  let op_token = parser.get_token();
  dbg!(&op_token);
  let mut compare_op: Option<CompareOp> = None;
  if op_token.is_eq() {
    compare_op = Some(CompareOp::Eq);
  } else if op_token.is_gt() {
    compare_op = Some(CompareOp::Gt);
  } else if op_token.is_lt() {
    compare_op = Some(CompareOp::Lt);
  }
  let mut right_expr = None;

  // skip = or > or <
  if compare_op.is_some() {
    parser.advance_token();
    right_expr = Some(parse_expression(parser));
  }

  CompareExpr::new(left_expr, compare_op, right_expr)
}


pub fn match_logic_expr(parser: &mut Parser) -> bool {
  let token = parser.get_token();
  match_compare_expr(parser) || token.is_and() || token.is_not() || token.is_or()
}

pub fn parse_logic_expr(parser: &mut Parser) -> LogicExpr {
  if !match_logic_expr(parser) {
    panic!("parse logic expr error, leading token not matched: {}", parser.get_token());
  }

  // only one <compare-expression>
  if match_compare_expr(parser) {
    let compare_expr = parse_compare_expr(parser);
    return LogicExpr::new(LogicOp::Bool, Some(compare_expr), None, None);
 }

  // handle and not or
  let token = parser.get_token();
  match token {
    Token::Not => {
      parser.eat_token(Token::Not);
      parser.eat_token(Token::LPar);
      
      let left_logic_expr = parse_logic_expr(parser);
      parser.eat_token(Token::RPar);
      LogicExpr::new(LogicOp::Not, None, Some(Box::new(left_logic_expr)), None)
    },

    Token::And => {
      parser.eat_token(Token::And);
      parser.eat_token(Token::LPar);
      
      let left_logic_expr = parse_logic_expr(parser);
      parser.eat_token(Token::Comma);
      let right_logic_expr = parse_logic_expr(parser);
      parser.eat_token(Token::RPar);

      LogicExpr::new(LogicOp::And, None, Some(Box::new(left_logic_expr)), Some(Box::new(right_logic_expr)))
    },

    Token::Or => {
      parser.eat_token(Token::Or);
      parser.eat_token(Token::LPar);
      
      let left_logic_expr = parse_logic_expr(parser);
      parser.eat_token(Token::Comma);
      let right_logic_expr = parse_logic_expr(parser);
      parser.eat_token(Token::RPar);

      LogicExpr::new(LogicOp::Or, None, Some(Box::new(left_logic_expr)), Some(Box::new(right_logic_expr)))
    },
    _ => panic!("match logic expr error, expect Not, And, Or token, but found {}", token)
  }

}


#[test]
fn test_parse_compare_expr() {
  let mut p = Parser::new("a > 1");
  let expr = parse_compare_expr(&mut p);
  dbg!(&expr);
  assert_eq!(matches!(expr.0.0, ExpressionValue::ArithmeticExpr(_)), true);
}


#[test]
fn test_parse_logic_expr() {
  let mut p = Parser::new("not(and(a > 1, b))");
  let expr = parse_logic_expr(&mut p);
  dbg!(&expr);
  assert_eq!(expr.0, LogicOp::Bool);
}
