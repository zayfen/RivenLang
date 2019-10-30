
use num_bigint::BigInt;

use std::fmt::{self, Write};

#[derive(Clone, Debug, PartialEq)]
pub enum NumberType {
  Int,
  Float,
  Complex
}

impl fmt::Display for NumberType {
  fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      NumberType::Int => write!(f, "int"),
      NumberType::Float => write!(f, "float"),
      NumberType::Complex => write!(f, "complex")
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
  // words
  Id { name: String },
  Number { number_type: NumberType, int: BigInt, float: f64 },
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
  Comma // ,
}

impl fmt::Display for Token {

  fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use Token::*;
    match self {
      Id { name } => write!(f, "'{}'", name),
      Number { int, float, number_type } => {
        match number_type {
          NumberType::Int => write!(f, "{}({})", number_type, int),
          NumberType::Float => write!(f, "{}({})", number_type, float),
          NumberType::Complex => write!(f, "{}({})", number_type, "Unsupport yet!"),
        }
      },
      String { value } => write!(f, "{}", value),
      Bool { value } => write!(f, "{}", value),
      Byte { value } => write!(f, "{}", value),
      Newline => f.write_str("NewLine"),
      EndOfFile => f.write_str("EndOfFile"),
      For => f.write_str("For"),
      In => f.write_str("In"),
      While => f.write_str("While"),
      If => f.write_str("If"),
      ElIf => f.write_str("Elif"),
      Else => f.write_str("Else"),
      Struct => f.write_str("Struct"),
      None => f.write_str("None"),
      True => f.write_str("True"),
      False => f.write_str("False"),
      Return => f.write_str("Return"),
      Break => f.write_str("Break"),
      Continue => f.write_str("Continue"),
      Plus => f.write_str("+"),
      Minus => f.write_str("-"),
      Star => f.write_str("*"),
      Slash => f.write_str("/"),
      Percent => f.write_str("%"),
      Vbar => f.write_str("|"),
      Amper => f.write_str("&"),
      Tilde => f.write_str("~"),
      Power => f.write_str("^"),
      LeftShift => f.write_str("<<"),
      RightShift => f.write_str(">>"),
      DoubleAmper => f.write_str("&&"),
      DoubleVbar => f.write_str("||"),
      Exclamation => f.write_str("!"),
      Greater => f.write_str(">"),
      Less => f.write_str("<"),
      Equal => f.write_str("="),
      DoubleEqual => f.write_str("=="),
      LPar => f.write_str("("),
      RPar => f.write_str(")"),
      LBracket => f.write_str("["),
      RBracket => f.write_str("]"),
      LBrace => f.write_str("{"),
      RBrace => f.write_str("}"),
      Dot => f.write_str("."),
      Semicolon => f.write_str(";"),
      Comma => f.write_str(",")
    }
  }
}
