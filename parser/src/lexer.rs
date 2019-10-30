use crate::location::Location;
use crate::error::{ LexicalError, LexicalErrorType };
pub use super::token::{Token, NumberType};
use num_bigint::BigInt;
use num_traits::identities::Zero;
use num_traits::Num;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;
use std::char;
use unicode_xid::UnicodeXID;


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


pub fn make_tokenizer<'a> (source: &'a str) -> impl Iterator<Item = LexResult> + 'a {
  let nlh = NewlineHandler::new(source.chars());
  let lch = LineContinationHandler::new(nlh);
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
    };
    nlh.shift();
    nlh.shift();
    nlh
  }

  pub fn shift (&mut self) -> Option<char> {
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

  fn next (&mut self) -> Option<Self::Item> {
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

// Glues \ and \n into a single line
pub struct LineContinationHandler<T: Iterator<Item = char>> {
  source: T,
  char0: Option<char>,
  char1: Option<char>,
}

impl<T> LineContinationHandler<T>
where
  T: Iterator<Item = char>
{
  pub fn new (source: T) -> Self {
    let mut nlh = LineContinationHandler {
      source,
      char0: None,
      char1: None
    };
    nlh.shift();
    nlh.shift();
    nlh
  }


  fn shift (&mut self) -> Option<char> {
    let c = self.char0;
    self.char0 = self.char1;
    self.char1 = self.source.next();
    c
  }
}

impl<T> Iterator for LineContinationHandler<T>
where
  T: Iterator<Item = char>
{
  type Item = char;

  fn next (&mut self) -> Option<Self::Item> {
    // Collapse \r\n into \n
    loop {
      if self.char0 == Some('\\') && self.char1 == Some('\n') {
        // skip backslash and newline
        self.shift();
        self.shift();
      } else {
        break;
      }
    }

    self.shift()
  }
}


impl<T> Lexer<T>
where
  T: Iterator<Item = char>
{
  pub fn new (input: T) -> Self {
    let mut lexer = Lexer {
      chars: input,
      at_begin_of_line: true,
      nesting: 0,
      pending: Vec::new(),
      char0: None,
      location: Location::new(0, 0),
      char1: None,
      char2: None,
      keywords: get_keywords()
    };

    lexer.next_char();
    lexer.next_char();
    lexer.next_char();

    lexer.location.reset();
    lexer
  }


  fn next_char (&mut self) -> Option<char> {
    let c = self.char0;
    let next = self.chars.next();
    self.char0 = self.char1;
    self.char1 = self.char2;
    self.char2 = next;
    if c == Some('\n') {
      self.location.newline();
    } else {
      self.location.forward();
    }

    c
  }

  fn get_pos (&self) -> Location {
    self.location.clone()
  }


  fn emit (&mut self, spanned: Spanned) {
    self.pending.push(spanned);
  }

  fn inner_next (&mut self) -> LexResult {
    while self.pending.is_empty() {
      self.consume_normal()?;
    }

    Ok(self.pending.remove(0))
  }

  fn is_identifier_start (&self, c: char) -> bool {
    match c {
      '_' => true,
      c => UnicodeXID::is_xid_start(c),
    }
  }

  fn is_identifier_continuation (&self) -> bool {
    if let Some(c) = self.char0 {
      match c {
        '_' | '0'..='9' => true,
        c => UnicodeXID::is_xid_continue(c),
      }
    } else {
       false
    }
  }

  fn consume_normal (&mut self) -> Result<(), LexicalError> {
    if let Some(c) = self.char0 {
      if self.is_identifier_start(c) {
        self.lex_identifier();
      } else {
        self.consume_character(c)?;
      }
      
    } else {
      // we reached end of file.
      let pos = self.get_pos();
 
      self.emit((pos.clone(), Token::EndOfFile, pos.clone()));
    }

    Ok(())
  }

  // resolve tokens

  fn lex_identifier (&mut self) -> LexResult {
    let mut name = String::new();
    let start_pos = self.get_pos().clone();
    while self.is_identifier_continuation() {
      name.push(self.next_char().unwrap());
    }
    let end_pos = self.get_pos().clone();

    if self.keywords.contains_key(&name) {
      Ok(( start_pos, self.keywords[&name].clone(), end_pos))
    } else {
      Ok(( start_pos, Token::Id { name: name }, end_pos ))
    }
  }


  fn lex_number (&mut self) -> LexResult {
    let start_pos = self.get_pos();
    if self.char0 == Some('0') {
      if self.char1 == Some('x') || self.char1 == Some('X') {
        // Hex
        self.next_char();
        self.next_char();
        self.lex_number_radix(start_pos, 16)

      } else if self.char1 == Some('o') || self.char1 == Some('O') {
        // Octal
        self.next_char();
        self.next_char();
        self.lex_number_radix(start_pos, 8)

      } else if self.char1 == Some('b') || self.char1 == Some('B') {
        // Binary
        self.next_char();
        self.next_char();
        self.lex_number_radix(start_pos, 2)

      } else {
        self.lex_normal_number()
      }
    } else {
      self.lex_normal_number()
    } 
  }

  fn lex_number_radix(&mut self, start_pos: Location, radix: u32) -> LexResult {
    let value_text = self.radix_run(radix);
    let end_pos = self.get_pos();
    let value = BigInt::from_str_radix(&value_text, radix).map_err(|e| LexicalError {
      error: LexicalErrorType::OtherError(format!("{:?}", e)),
      location: start_pos.clone(),
    })?;
    Ok((start_pos, Token::Number { int: value, number_type: NumberType::Int, float: 0f64 }, end_pos))
  }

  fn radix_run (&mut self, radix: u32) -> String {
    let mut value_text = String::new();

    loop {
      if let Some(c) = self.take_number(radix) {
        value_text.push(c);

      } else if self.char0 == Some('_') && Lexer::<T>::is_digit_of_radix(self.char1, radix) {
        self.next_char();

      } else {
        break;
      }
    }

    value_text
  }

  /// Consume a single character with the given radix.
  fn take_number(&mut self, radix: u32) -> Option<char> {
    let take_char = Lexer::<T>::is_digit_of_radix(self.char0, radix);

    if take_char {
      Some(self.next_char().unwrap())
    } else {
      None
    }
  }

  /// Test if a digit is of a certain radix.
  fn is_digit_of_radix(c: Option<char>, radix: u32) -> bool {
    match radix {
      2 => match c {
        Some('0'..='1') => true,
        _ => false,
      },
      8 => match c {
        Some('0'..='7') => true,
        _ => false,
      },
      10 => match c {
        Some('0'..='9') => true,
        _ => false,
      },
      16 => match c {
        Some('0'..='9') | Some('a'..='f') | Some('A'..='F') => true,
        _ => false,
      },
      x => unimplemented!("Radix not implemented: {}", x),
    }
  }

  fn at_exponent (&self) -> bool {
    match self.char0 {
      Some('e') | Some('E') => match self.char1 {
        Some('+') | Some('-') => match self.char2 {
          Some('0'..='9') => true,
          _ => false
        },
        Some('0'..='9') => true,
        _ => false,
      },

      _ => false,
    }
  }

  fn lex_normal_number (&mut self) -> LexResult {
    let start_pos = self.get_pos();
    let start_is_zero = self.char0 == Some('0');
    let mut value_text = self.radix_run(10);

    if self.char0 == Some('.') || self.at_exponent() {

      if self.char0 == Some('.') {
        if self.char1 == Some('_') {
          return Err(LexicalError {
            error: LexicalErrorType::OtherError("Invalid Syntax".to_owned()),
            location: self.get_pos()
          })
        }
        value_text.push(self.next_char().unwrap());
        value_text.push_str(&self.radix_run(10));
      }

      if self.char0 == Some('e') || self.char0 == Some('E') {
        value_text.push(self.next_char().unwrap().to_ascii_lowercase());

        if self.char0 == Some('-') || self.char0 == Some('+') {
          value_text.push(self.next_char().unwrap());
        }

        value_text.push_str(&self.radix_run(10));
      }

      let value = f64::from_str(&value_text).unwrap();
      let end_pos = self.get_pos();
      Ok((start_pos, Token::Number { number_type: NumberType::Float, float: value, int: BigInt::from_str("0").unwrap() }, end_pos ))

    } else {
      let end_pos = self.get_pos();
      let value = value_text.parse::<BigInt>().unwrap();
      if start_is_zero && !value.is_zero() {
        return Err(LexicalError {
          error: LexicalErrorType::OtherError("Invalid Token".to_string()),
          location: self.get_pos(),
        })
      }
      Ok((start_pos, Token::Number{ number_type: NumberType::Int, int: value, float: 0f64 }, end_pos))
    }
  }


  fn lex_string (&mut self) -> LexResult {
    let quote_char = self.next_char().unwrap();
    let mut value_text = String::new();

    loop {
      match self.next_char() {
        Some('\\') => {
          if self.char0 == Some(quote_char) {
            value_text.push(quote_char);
            self.next_char();

          } else {
            match self.next_char() {
              Some('\\') => {
                value_text.push('\\');
              }
              Some('\'') => value_text.push('\''),
              Some('\"') => value_text.push('\"'),
              Some('\n') => {
                // Ignore Unix EOL character
              }
              Some('a') => value_text.push('\x07'),
              Some('b') => value_text.push('\x08'),
              Some('f') => value_text.push('\x0c'),
              Some('n') => {
                value_text.push('\n');
              }
              Some('r') => value_text.push('\r'),
              Some('t') => {
                value_text.push('\t');
              }
              Some('u') => value_text.push(self.unicode_literal(4)?),
              Some('U') => value_text.push(self.unicode_literal(8)?),
              Some('x') if !is_bytes => value_text.push(self.unicode_literal(2)?),
              Some('v') => value_text.push('\x0b'),
              Some(c) => {
                value_text.push('\\');
                value_text.push(c);
              }
              None => {
                return Err(LexicalError {
                  error: LexicalErrorType::StringError,
                  location: self.get_pos(),
                });
              }
              
            }
          },

          Some(c) => {
            if c == quote_char {
              break;

            } else {
              if c == '\n' {
                return Err(LexicalError {
                  error: LexicalErrorType::StringError,
                  location: self.get_pos(),
                })
              }
            }
          }
        }
      }
    }


    fn consume_character (&mut self, c: char) -> Result<(), LexicalError> {
      match c {
        '0'..='9' => {
          // number
          let number = self.lex_number()?;
          self.emit(number);
        },
        '"' | '\'' => {
          // string
          let string = self.lex_string()?;
          self.emit(string);
        }
      }
      Ok(())
    }
    
  }


impl<T> Iterator for Lexer<T>
where
  T: Iterator<Item = char>
{
  type Item = LexResult;

  fn next (&mut self) -> Option<Self::Item> {
    let token = self.inner_next();
    trace!("Lex token {:?}, nesting={:?}", token, self.nesting);
    match token {
      Ok((_, Token::EndOfFile, _)) => None,
      result => Some(result),
    }
  }
}




