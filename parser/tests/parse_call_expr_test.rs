use parser::{ast::Identifier, parse_call_expr::parse_call_expr, parser::Parser};

#[cfg(test)]
mod parser_test {
  use super::*;

  #[test]
  fn test_parse_call_expr() {
    let mut parser = Parser::new("fn(id, name)");
    let call_expr = parse_call_expr(&mut parser);
    assert_eq!(
      format!("{:?}", call_expr),
      "CallExpr(Identifier(\"fn\"), [Identifier(\"id\"), Identifier(\"name\")])"
    );
    // assert_eq!(call_expr.0, Identifier::from("fn"));
    // assert_eq!(
    //   call_expr.1,
    //   vec![Identifier::from("id"), Identifier::from("name")]
    // );
  }
}
