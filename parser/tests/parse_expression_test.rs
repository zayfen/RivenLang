use parser::parse_expression::parse_expression;
use parser::parser::Parser;

#[cfg(test)]
mod parser_test {
  use super::*;

  #[test]
  pub fn test_parse_expression_callexpr() {
    let mut parser = Parser::new("fn(id, name)");
    let expression = parse_expression(&mut parser);

    assert_eq!(
      format!("{:?}", expression),
      "Expression(CallExpr(CallExpr(Identifier(\"fn\"), [Identifier(\"id\"), Identifier(\"name\")])))"
    );
  }

  #[test]
  fn test_parse_expression_arithmetic() {
    let mut parser = Parser::new("1 + 2 * 3");
    let expression = parse_expression(&mut parser);
    assert_eq!(format!("{:?}", expression), "Expression(ArithmeticExpr(ArithmeticExpr(Term(Factor(Primary(Primary(Number(1.0)))), None, None), Some(Add), Some(ArithmeticExpr(Term(Factor(Primary(Primary(Number(2.0)))), Some(Time), Some(Term(Factor(Primary(Primary(Number(3.0)))), None, None))), None, None)))))");
  }
}
