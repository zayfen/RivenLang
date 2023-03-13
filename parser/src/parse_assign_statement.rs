use crate::{token::Token, parser::Parser, ast::{AssignStmt, Identifier}, parse_expression::parse_expression};

pub fn match_assign_stmt(parser: &mut Parser) -> bool {
  let token = parser.get_token();
  let next_token = parser.peek_token();
  token.is_id() && next_token.is_eq()
}

pub fn parse_assign_stmt(parser: &mut Parser) -> AssignStmt {

  if !match_assign_stmt(parser) {
    panic!("parse_assign_stmt error");
  }

  let token = parser.get_token();

  let id = if let Token::Id { name } = token {
    Identifier::from(name.as_str())
  } else {
    panic!("parse assign statement error, missing identifier")
  };

  parser.advance_token();
  // now cursor point to Equal
  parser.advance_token();
  // now cursor point to Expressinon

  let expr = parse_expression(parser);

  parser.eat_token(Token::Semicolon);

  AssignStmt(id, expr)
}


#[test]
fn test_parse_assign_stmt() {
  let code = "name = 1 + 2;";
  let mut parser = Parser::new(code);
  let assign_stmt = parse_assign_stmt(&mut parser);
  assert_eq!(assign_stmt.0, Identifier::from("name"));

  let expr_code = "1 + 2";
  let mut parser2 = Parser::new(expr_code);
  let expr = parse_expression(&mut parser2);
  assert_eq!(assign_stmt.1, expr);
}
