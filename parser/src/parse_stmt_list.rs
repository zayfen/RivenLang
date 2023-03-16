use crate::{ast::StmtList, parser::Parser, parse_stmt::{parse_stmt, match_parse_stmt}};



pub fn parse_stmt_list(parser: &mut Parser) -> StmtList {
  
  if !match_parse_stmt(parser) {
    // empty statement, like  {.}   (cursor between '{' and '}')
    return StmtList(None, None);
  }

  let stmt = parse_stmt(parser);

  if parser.get_token().is_rbrace() {
    return StmtList(Some(stmt), None);
  }

  StmtList(Some(stmt), Some(Box::new(parse_stmt_list(parser))))
}


#[test]
pub fn test_parse_stmt_list() {
  let code = "fn foo() {} name = value;";
  let mut parser = Parser::new(code);
  let stmt_list = parse_stmt_list(&mut parser);
}
