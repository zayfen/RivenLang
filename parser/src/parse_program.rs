use crate::{parser::Parser, ast::Program, token::Token, parse_stmt_list::parse_stmt_list};


pub fn parse_program(parser: &mut Parser) -> Program {
  parser.eat_token(Token::Program);

  parser.eat_token(Token::LBrace);
  let stmt_list = parse_stmt_list(parser);
  parser.eat_token(Token::RBrace);

  Program(stmt_list)
}
