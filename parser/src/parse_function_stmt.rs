use crate::{parser::Parser, ast::{FunctionStmt, Identifier, StmtList}, token::Token, parse_stmt_list::parse_stmt_list};


pub fn match_function_stmt(parser: &mut Parser) -> bool {
  parser.get_token().is_keyword_function()
}

fn eat_token(parser: &mut Parser, token: Token) {
  let curr_token = parser.get_token();
  if curr_token.to_string() != token.to_string() {
    panic!("token not matched");
  }

  parser.advance_token();
}

pub fn parse_function_stmt(parser: &mut Parser) -> FunctionStmt {
  eat_token(parser, Token::Function);
  
  let fun_name = if let Token::Id { name } = parser.get_token() {
    Identifier::from(name.as_str())
  } else {
    panic!("parse funtion name error: mission function name")
  };

  eat_token(parser, Token::LPar);
  let mut params: Vec<Identifier> = vec![];

  let mut token = parser.get_token();
  while token.is_id() {
    if let Token::Id { name } = token {
      params.push(Identifier::from(name.as_str()));
    }
    // ,
    token = parser.advance_token();
    if token.is_comma() {
      token = parser.advance_token();
    }
  }

  eat_token(parser, Token::RPar);
  eat_token(parser, Token::LBrace);

  // TODO: parse statement list
  let stmt_list = parse_stmt_list(parser);
  eat_token(parser, Token::RBrace);

  FunctionStmt::new(fun_name, params, stmt_list)
}
