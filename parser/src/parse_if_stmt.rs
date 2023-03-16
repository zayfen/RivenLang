use crate::{
  ast::{IfStmt, StmtList},
  parse_logic_expr::parse_logic_expr,
  parse_stmt_list::parse_stmt_list,
  parser::Parser,
  token::Token,
};

pub fn match_if_stmt(parser: &mut Parser) -> bool {
  let token = parser.get_token();
  token.is_keyword_if()
}

pub fn parse_if_stmt(parser: &mut Parser) -> IfStmt {
  if !match_if_stmt(parser) {
    panic!("parse if statement error: missing IF keyword");
  }

  parser.advance_token();
  // now cursor point to (
  if !parser.get_token().is_lpar() {
    panic!("parse if statment error: missing LPAR('(')");
  }

  parser.advance_token();
  let expr = parse_logic_expr(parser);

  // now cursor point to )
  if !parser.get_token().is_rpar() {
    panic!("parse if statement error: missing RPAR(')')");
  }

  parser.advance_token();
  // now cursor point to lbrace {
  if !parser.get_token().is_lbrace() {
    panic!("parse if statement error: missing lbrace {");
  }

  parser.advance_token();

  let stmt_list = parse_stmt_list(parser);

  // now cursor point to rbrace }
  if !parser.get_token().is_rbrace() {
    panic!("parse if statement error: missing rbrace {}", "}")
  }

  parser.eat_token(Token::RBrace);
  IfStmt::new(expr, stmt_list)
}

#[test]
fn test_if_stmt() {
  let code = "if (not(1 + 2 > 0)) { name = \"zayfen\"; foo(name); fn foo2 (nage) {a = 1+2; b = 2+4; print(a,b);}}";
  let mut parser = Parser::new(code);
  let if_stmt = parse_if_stmt(&mut parser);
  println!("{:?}", if_stmt);

  let expr_code = "not(1 + 2 > 0)";
  let mut parser2 = Parser::new(expr_code);
  let expr = parse_logic_expr(&mut parser2);
  assert_eq!(if_stmt.0, expr);
}
