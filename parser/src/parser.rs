use crate::lexer::{LexResult, Lexer, LineContinationHandler, NewlineHandler};
use crate::token::Token;
use std::str::Chars;

#[derive(Debug)]
pub struct Parser<'a> {
  lex: Lexer<LineContinationHandler<NewlineHandler<Chars<'a>>>>,
  next_token: Token,
  nnext_token: Token,
}

fn advance_token(lex: &mut Lexer<LineContinationHandler<NewlineHandler<Chars>>>) -> Token {
  let result: Option<LexResult> = lex.next();
  // println!("Option<LexResult>: {:?}", result);
  // println!("next_token <<<<<<<<<<<<<<<<<");

  if result.is_none() {
    return Token::None;
  }
  let result = result.unwrap();
  match result {
    Ok(v) => v.1,
    Err(_) => Token::None,
  }
}

impl<'a> Parser<'a> {
  pub fn new(source: &'a str) -> Self {
    let nlh = NewlineHandler::new(source.chars());
    let lch = LineContinationHandler::new(nlh);
    let mut lex = Lexer::new(lch);

    let mut next_token = advance_token(&mut lex);
    while next_token.is_newline() || next_token.is_comment() {
      next_token = advance_token(&mut lex);
    }
    
    let mut next_next_token = advance_token(&mut lex);
    while next_next_token.is_newline() || next_next_token.is_comment() {
      next_next_token = advance_token(&mut lex);
    }

    Parser {
      lex,
      next_token,
      nnext_token: next_next_token,
    }
  }

  // get next token and advances the current token
  pub fn advance_token(&mut self) -> Token {
    self.next_token = self.nnext_token.clone();
    while self.next_token.is_newline() || self.next_token.is_comment() {
      self.next_token = advance_token(&mut self.lex);
    }

    self.nnext_token = advance_token(&mut self.lex);
    while self.nnext_token.is_newline() || self.nnext_token.is_comment() {
      self.nnext_token = advance_token(&mut self.lex);
    }

    self.get_token()
  }

  // get current token
  pub fn get_token(&mut self) -> Token {
    self.next_token.clone()
  }

  // return true if current token matches.
  pub fn check_token(&mut self, token: &Token) -> bool {
    self.next_token.to_string() == token.to_string()
  }

  pub fn eat_token(&mut self, token: Token) {

    if !self.check_token(&token) {
      panic!("token dont match, expect {:?}, but found {:?}", token, self.next_token.clone());
    }

    self.advance_token();
  }

  // get next token but don't advance
  pub fn peek_token(&mut self) -> Token {
    self.nnext_token.clone()
  }

  // return true if next token matches.
  pub fn check_peek(&mut self, token: Token) -> bool {
    self.peek_token().to_string() == token.to_string()
  }

}
