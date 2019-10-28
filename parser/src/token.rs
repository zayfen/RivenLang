
use num_bigint::BigInt;

use std::fmt::{self, Write};


#[derive(Clone, Debug, PartialEq)]
pub enum Token {
  // words
  Id { name: String },
  Number { value: BigInt },
  String { value: String },
  Bool { value: bool },
  Byte { value: u8 },
  Newline,
  EndOfFile,
  For,
  In,
  While,
  If,
  ElIf,
  Else,
  Struct,
  None,
  True,
  False,
  Return,
  Break,
  Continue,
  // symboles
  Plus, // +
  Minus, // -
  Star, // *
  Slash, // /
  Percent, // %
  Vbar, // |
  Amper, // &
  Tilde, // ~
  Power, // ^
  LeftShift, // <<
  RightShift, // >>
  DoubleAmper, // &&
  DoubleVbar, // ||
  Exclamation, // !
  Greater, // >
  Less, // <
  Equal, // =
  DoubleEqual, // ==
  LPar, // (
  RPar, // )
  LBracket, // [
  RBracket, // ]
  LBrace, // {
  RBrace, // }
  Dot, // .
  Semicolon, // ;
  Comma // ;
}

impl fmt::Display for Token {

  fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use Token::*;
    match self {
      Id { name } => write!(f, "'{}'", name),
      Number { value } => write!(f, "'{}'", value),
      String { value } => write!(f, "{}", value),
      Bool { value } => write!(f, "{}", value),
      Byte { value } => write!(f, "{}", value),
      Newline => f.write_str("NewLine"),
      EndOfFile => f.write_str("EndOfFile"),
      For => f.write_str("For"),
      In => f.write_str("In"),
      While => f.write_str("While"),
      
    }
  }
}
