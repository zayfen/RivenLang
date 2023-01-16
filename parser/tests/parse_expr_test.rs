use parser::{ast::EnumBinaryOperators, parse_expression::parse_arithmetic_expr, parser::Parser};

#[test]
fn test_arithmetic_expr() {
  let mut parser = Parser::new("123 * 456 + 17 / 20 - 100");
  let expr = parse_arithmetic_expr(&mut parser);
  println!("expr {:?}", expr.clone());
  assert_eq!(expr.unwrap().1, Some(EnumBinaryOperators::Add));
}
