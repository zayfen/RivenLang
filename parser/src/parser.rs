use crate::ast::Program;
use crate::lexer::{Lexer, LexResult, LineContinationHandler, NewlineHandler};
use crate::token::Token;
use std::str::Chars;

#[derive(Debug)]
pub struct Parser<'a> {
  lex: Lexer<LineContinationHandler<NewlineHandler<Chars<'a>>>>,
  lookahead: Token,
}

impl<'a> Parser<'a> {
  pub fn new(source: &'a str) -> Self {
    let nlh = NewlineHandler::new(source.chars());
    let lch = LineContinationHandler::new(nlh);
    let lex = Lexer::new(lch);

    Parser {
      lex: lex,
      lookahead: Token::None,
    }
  }

  pub fn next_token (&mut self) -> Token {
    let result: Option<LexResult> = self.lex.next();
    let result = result.map(|lr| lr).unwrap();
    match result {
      Ok(v) => v.1,
      Err(_) => Token::None
    }
  }

  pub fn next (&mut self) -> Token {
    let token = self.next_token();
    let next_token = self.next_token();
    self.lookahead = next_token;
    token
  }
}
