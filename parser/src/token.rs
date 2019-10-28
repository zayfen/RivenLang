
use num_bigint::BigInt;

use std::fmt::{self, Write};


#[derive(Clone, Debug, PartialEq)]
pub enum Token {
  // words
  Id { name: String },
  Number { value: BigInt },
  String { value: String, is_fstring: bool },
  Bool { value: bool },
  Byte { value: u8 },
  Newline,
  EndOfFile,
  For,
  In,
  While,
  If,
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

