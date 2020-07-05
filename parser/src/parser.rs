use crate::lexer::{ Lexer, NewlineHandler, LineContinationHandler };
use crate::token::Token;
use crate::ast::Program;

pub struct Parser<T: Iterator<Item = char>> {
  lex: Lexer<T>,
  lookahead: Token
}

impl<T> Parser<T>
  where T: Iterator<Item = char> {
  fn new (source: &str) -> Self {
    let nlh = NewlineHandler::new(source.chars());
    let lch = LineContinationHandler::new(nlh);
    let lex = Lexer::new(lch);

    Parser {
      lex: lex,
      lookahead: Token::None
    }
  }

  pub fn parse () -> Program {
  }
}
