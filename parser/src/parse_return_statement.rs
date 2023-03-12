use crate::{token::Token, parser::Parser, ast::ReturnStmt, parse_expression::parse_expression};



pub fn match_return_stmt(token: Token) -> bool {
  matches!(token, Token::Return)
}

pub fn parse_return_stmt(parser: &mut Parser) -> ReturnStmt {
  let token = parser.get_token();
  parser.eat_token(Token::Return);

  let expr = parse_expression(parser);

  // now cursor should point to semicolon
  if !parser.get_token().is_semi() {
    panic!("parse_return_statement error: missing semicolon");
  }
  parser.advance_token();

  ReturnStmt::new(expr)
}


#[test]
fn test_parse_return_stmt() {
  let code = "return 1 + 2;";
  let mut parser = Parser::new(code);
  let return_stmt = parse_return_stmt(&mut parser);

  let expr_code = "1 + 2";
  let mut parser2 = Parser::new(expr_code);
  let expr = parse_expression(&mut parser2);
  assert_eq!(return_stmt.0, expr);
}
