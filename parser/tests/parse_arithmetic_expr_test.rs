use parser::{
  ast::{ArithmeticExpr, BinOp, Factor, FactorValue, Identifier, Term},
  parse_arithmetic_expr::parse_arithmetic_expr,
  parser::Parser,
};

#[test]
fn test_parse_arithmetic_expr() {
  let mut parser = Parser::new("name + 3 / 2 * count");
  let token = parser.get_token();
  let arith_expr = parse_arithmetic_expr(&mut parser);
  let factor = Factor::from(FactorValue::Identifier(Identifier::from("name")));

  assert_eq!(arith_expr.0, Term::new(factor, Some(BinOp::Add), None));

  println!("arith_expr is: {:?}", arith_expr);
}
