use crate::lexer::{LexResult, Lexer, LineContinationHandler, NewlineHandler};
use crate::token::Token;
use std::str::Chars;

#[derive(Debug)]
pub struct Parser<'a> {
  lex: Lexer<LineContinationHandler<NewlineHandler<Chars<'a>>>>,
}

impl<'a> Parser<'a> {
  pub fn new(source: &'a str) -> Self {
    let nlh = NewlineHandler::new(source.chars());
    let lch = LineContinationHandler::new(nlh);
    let lex = Lexer::new(lch);

    Parser { lex }
  }

  pub fn next_token(&mut self) -> Token {
    println!("next_token >>>>>>>>>>>>>>>>>>");
    let result: Option<LexResult> = self.lex.next();
    println!("Option<LexResult>: {:?}", result);
    println!("next_token <<<<<<<<<<<<<<<<<");

    if result.is_none() {
      return Token::None;
    }
    let result = result.unwrap();
    match result {
      Ok(v) => v.1,
      Err(_) => Token::None,
    }
  }

  // lookforward 2 tokens
  pub fn lookahead2(&mut self) -> (Token, Token) {
    let token = self.next_token();
    let next_token = self.next_token();
    (token, next_token)
  }
}
