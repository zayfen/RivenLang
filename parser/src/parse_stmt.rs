use crate::{parser::Parser, ast::{Statement, StatementValue}, parse_if_stmt::parse_if_stmt, parse_function_stmt::parse_function_stmt, parse_return_statement::parse_return_stmt, parse_assign_statement::parse_assign_stmt, parse_call_expr::parse_call_expr, token::Token};



pub fn parse_stmt(parser: &mut Parser) -> Statement {
    // assign stmt:   id = 
    // call stmt:  id (
    // return stmt: return 
    // function stmt: function 
    // if stmt: if (
    let token = parser.get_token();
    let next_token = parser.peek_token();

    if token.is_keyword_if() {
      return Statement::from(StatementValue::IfStmt(parse_if_stmt(parser)));

    } else if token.is_keyword_function() {
        return Statement::from(StatementValue::FunctionStmt(parse_function_stmt(parser)));

    } else if token.is_keyword_return() {
      return Statement::from(StatementValue::ReturnStmt(parse_return_stmt(parser)));

    } else if token.is_id() && next_token.is_eq() {
      return Statement::from(StatementValue::AssignStmt(parse_assign_stmt(parser)));
      
    } else if token.is_id() && next_token.is_lpar() {
      let expr = parse_call_expr(parser);
      parser.eat_token(Token::Semicolon);
      return Statement::from(StatementValue::CallStmt(expr));
    }
    
    panic!("parse stmt error, token is {}", token.to_string());
}
