use crate::lexer::{ Lexer, NewlineHandler, LineContinationHandler };
use crate::token::Token;
use crate::ast::Program;
use std::str::Chars;

pub struct Parser<'a> {
  lex: Lexer<LineContinationHandler<NewlineHandler<Chars<'a>>>>,
  lookahead: Token
}

impl<'a> Parser<'a> {
  fn new (source: &'a str) -> Self {
    let nlh = NewlineHandler::new(source.chars());
    let lch = LineContinationHandler::new(nlh);
    let lex = Lexer::new(lch);

    Parser {
      lex: lex,
      lookahead: Token::None
    }
  }

}
