use crate::{parser::Parser, ast::Program, token::Token, parse_stmt_list::parse_stmt_list, parse_expression::parse_expression};


pub fn parse_program(parser: &mut Parser) -> Program {
  parser.eat_token(Token::Program);

  parser.eat_token(Token::LBrace);
  let stmt_list = parse_stmt_list(parser);
  parser.eat_token(Token::RBrace);

  Program(stmt_list)
}

#[test]
fn test_parse_program() {
  let code = "program { if (1 + 2) { name = \"zayfen\"; foo(name); function foo2 (nage) {a = 1+2; b = 2+4; print(a,b);}}}";
  let mut parser = Parser::new(code);
  let program_stmt = parse_program(&mut parser);
  println!("{:?}", program_stmt);
  
}
