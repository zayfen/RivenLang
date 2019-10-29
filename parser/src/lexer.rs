use crate::location::Location;
use crate::error::{ LexicalError, LexicalErrorType };
pub use super::token::Token;
use num_bigint::BigInt;
use num_traits::identities::Zero;
use num_traits::Num;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;
use std::char;


pub type Spanned = (Location, Token, Location);
pub type LexResult = Result<Spanned, LexicalError>;



pub struct Lexer<T: Iterator<Item = char>> {
  chars: T,
  at_begin_of_line: bool,
  nesting: usize, // amout of parenthesis
  pending: Vec<Spanned>,
  char0: Option<char>,
  char1: Option<char>,
  char2: Option<char>,
  location: Location,
  keywords: HashMap<String, Token>
}


pub fn get_keywords () -> HashMap<String, Token> {
  let mut keywords: HashMap<String, Token> = HashMap::new();

  // keywords
  keywords.insert(String::from("for"), Token::For);
  keywords.insert(String::from("in"), Token::In);
  keywords.insert(String::from("while"), Token::While);
  keywords.insert(String::from("if"), Token::If);
  keywords.insert(String::from("elif"), Token::ElIf);
  keywords.insert(String::from("else"), Token::Else);
  keywords.insert(String::from("struct"), Token::Struct);
  keywords.insert(String::from("None"), Token::None);
  keywords.insert(String::from("True"), Token::True);
  keywords.insert(String::from("False"), Token::False);
  keywords.insert(String::from("return"), Token::Return);
  keywords.insert(String::from("break"), Token::Break);
  keywords.insert(String::from("continue"), Token::Continue);

  keywords
}


pub fn make_tokenizer<'a> (source: &'a str) -> impl Iterator<LexResult> + 'a {
  let nlh = NewlineHandler::new(source.chars());
  let lch = LineContainerHandler::new(nlh);
  Lexer::new(lch)
}


pub struct NewlineHandler<T: Iterator<Item = char>> {
  source: T,
  char0: Option<char>,
  char1: Option<char>,
}

impl<T> NewlineHandler<T>
where T: Iterator<Item = char>
{
  pub fn new (source: T) -> Self {
    let mut nlh = NewlineHandler {
      source,
      char0: None,
      char1: None,
    }
    nlh.shift();
    nlh.shift();
    nlh
  }

  pub fn shift (&mut self) Option<char> {
    let c = self.char0;
    self.char0 = self.char1;
    self.char1 = self.source.next();
    c
  }
}


impl<T> Iterator for NewlineHandler<T>
where T: Iterator<Item = char>
{
  type Item = char;
  pub fn next (&mut self) -> Option<Self::Item> {
    loop {
      if self.char0 == Some('\r') {

        if self.char1 == Some('\n') {
          // Transform windows EOL into \n
          self.shift();
        } else {
          // Transform MAC EOL into \n
          self.char0 = Some('\n');
        }
      } else {
        break;
      }
    }

    self.shift()
  }
  
}



