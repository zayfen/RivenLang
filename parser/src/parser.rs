use crate::ast::Program;
use crate::lexer::{Lexer, LineContinationHandler, NewlineHandler};
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
}
