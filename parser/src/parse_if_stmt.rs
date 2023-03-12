use crate::{parser::Parser, ast::{IfStmt, StmtList}, parse_expression::parse_expression, parse_stmt_list::parse_stmt_list};


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
  let expr = parse_expression(parser);

  // now cursor point to )
  if !parser.get_token().is_rpar() {
    panic!("parse if statement error: missing RPAR(')')");
  }

  parser.advance_token();
  // now cursor point to lbrace {
  if !parser.get_token().is_lbrace() {
    panic!("parse if statement error: missing lbrace {");
  }

  // TODO: parse_statement_list
  let stmt_list = parse_stmt_list(parser);

  // now cursor point to rbrace }
  if !parser.get_token().is_rbrace() {
    panic!("parse if statement error: missing rbrace }")
  }

  IfStmt::new(expr, stmt_list)
}
