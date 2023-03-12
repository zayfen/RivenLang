use crate::{ast::StmtList, parser::Parser, parse_stmt::parse_stmt};



pub fn parse_stmt_list(parser: &mut Parser) -> StmtList {
  if parser.get_token().is_rbrace() {
    // empty statement, like  {.}   (cursor between '{' and '}')
    parser.advance_token();
    return StmtList(None, None);
  }  

  let stmt = parse_stmt(parser);

  if parser.get_token().is_rbrace() {
    return StmtList(Some(stmt), None);
  }

  StmtList(Some(stmt), Some(Box::new(parse_stmt_list(parser))))
}
