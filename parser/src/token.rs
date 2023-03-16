use num_bigint::BigInt;
use std::fmt::{self, Write};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NumberType {
  Int,
  Float,
  Complex,
}

impl NumberType {
  pub fn is_int(&self) -> bool {
    matches!(*self, NumberType::Int)
  }

  pub fn is_float(&self) -> bool {
    matches!(*self, NumberType::Float)
  }
}

impl fmt::Display for NumberType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      NumberType::Int => write!(f, "int"),
      NumberType::Float => write!(f, "float"),
      NumberType::Complex => write!(f, "complex"),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
  // words
  Id {
    name: String,
  },
  Number {
    number_type: NumberType,
    int: BigInt,
    float: f64,
  },
  String {
    value: String,
  },
  Bool {
    value: bool,
  },
  Byte {
    value: u8,
  },
  Not,
  And,
  Or,
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
  Return,
  Break,
  Continue,
  Function,
  Program,
  Comment {
    value: String,
  },
  // symboles
  Plus,        // +
  Minus,       // -
  Star,        // *
  Slash,       // /
  Percent,     // %
  Vbar,        // |
  Amper,       // &
  Tilde,       // ~
  Power,       // ^
  LeftShift,   // <<
  RightShift,  // >>
  DoubleAmper, // &&
  DoubleVbar,  // ||
  Exclamation, // !
  Greater,     // >
  Less,        // <
  Equal,       // =
  DoubleEqual, // ==
  LPar,        // (
  RPar,        // )
  LBracket,    // [
  RBracket,    // ]
  LBrace,      // {
  RBrace,      // }
  Dot,         // .
  Semicolon,   // ;
  Comma,       // ,
}

impl Token {
  pub fn is_number(&self) -> bool {
    matches!(
      self.clone(),
      Token::Number {
        number_type: _,
        int: _,
        float: _
      }
    )
  }

  pub fn is_string(&self) -> bool {
    matches!(self, Token::String { value: _ })
  }

  pub fn is_star(&self) -> bool {
    matches!(self, Token::Star)
  }

  pub fn is_slash(&self) -> bool {
    matches!(self, Token::Slash)
  }

  pub fn is_plus(&self) -> bool {
    matches!(self, Token::Plus)
  }

  pub fn is_minus(&self) -> bool {
    matches!(self, Token::Minus)
  }

  pub fn is_lpar(&self) -> bool {
    matches!(self, Token::LPar)
  }

  pub fn is_rpar(&self) -> bool {
    matches!(self, Token::RPar)
  }

  pub fn is_lbrace(&self) -> bool {
    matches!(self, Token::LBrace)
  }

  pub fn is_rbrace(&self) -> bool {
    matches!(self, Token::RBrace)
  }

  pub fn is_id(&self) -> bool {
    matches!(self, Token::Id { name: _ })
  }

  pub fn is_comma(&self) -> bool {
    matches!(self, Token::Comma)
  }

  pub fn is_eq(&self) -> bool {
    matches!(self, Token::Equal)
  }

  pub fn is_semi(&self) -> bool {
    matches!(self, Token::Semicolon)
  }

  pub fn is_keyword_return(&self) -> bool {
    matches!(self, Token::Return)
  }

  pub fn is_keyword_if(&self) -> bool {
    matches!(self, Token::If)
  }

  pub fn is_keyword_function(&self) -> bool {
    matches!(self, Token::Function)
  }

  pub fn is_eof(&self) -> bool {
    matches!(self, Token::EndOfFile)
  }

  pub fn is_not(&self) -> bool {
    matches!(self, Token::Not)
  }

  pub fn is_and(&self) -> bool {
    matches!(self, Token::And)
  }

  pub fn is_or(&self) -> bool {
    matches!(self, Token::Or)
  }

  pub fn is_gt(&self) -> bool {
    matches!(self, Token::Greater)
  }

  pub fn is_lt(&self) -> bool {
    matches!(self, Token::Less)
  }

  pub fn is_newline(&self) -> bool {
    matches!(self, Token::Newline)
  }

  pub fn is_comment(&self) -> bool {
    matches!(self, Token::Comment { value: _ })
  }


}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use Token::*;
    match self {
      Id { name } => write!(f, "'{}'", name),
      Number {
        int,
        float,
        number_type,
      } => match number_type {
        NumberType::Int => write!(f, "{}({})", number_type, int),
        NumberType::Float => write!(f, "{}({})", number_type, float),
        NumberType::Complex => write!(f, "{}(Unsupport yet!)", number_type),
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
      Return => f.write_str("Return"),
      Break => f.write_str("Break"),
      Continue => f.write_str("Continue"),
      Comment { value } => write!(f, "{}", value),
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
      Comma => f.write_str(","),
      Function => f.write_str("FUNCTION"),
      Program => f.write_str("MAIN"),
      Not => f.write_str("!"),
      And => f.write_str("&&"),
      Or => f.write_str("||")
    }
  }
}

pub fn get_token_kind(token: Token) -> String {
  use Token::*;

  match token {
    Id { name } => "Id".to_owned(),
    Number {
      int,
      float,
      number_type,
    } => "Number".to_owned(),
    String { value } => "String".to_owned(),
    Bool { value } => "Bool".to_owned(),
    Byte { value } => "Byte".to_owned(),
    Newline => "Newline".to_owned(),
    EndOfFile => "EndOfFile".to_owned(),
    For => "For".to_owned(),
    In => "In".to_owned(),
    While => "While".to_owned(),
    If => "If".to_owned(),
    ElIf => "ElIf".to_owned(),
    Else => "Else".to_owned(),
    Struct => "Struct".to_owned(),
    None => "None".to_owned(),
    Return => "Return".to_owned(),
    Break => "Break".to_owned(),
    Continue => "Continue".to_owned(),
    Comment { value } => "Comment".to_owned(),
    Plus => "Plus".to_owned(),
    Minus => "Minus".to_owned(),
    Star => "Star".to_owned(),
    Slash => "Slash".to_owned(),
    Percent => "Percent".to_owned(),
    Vbar => "Vbar".to_owned(),
    Amper => "Amper".to_owned(),
    Tilde => "Tilde".to_owned(),
    Power => "Power".to_owned(),
    LeftShift => "LeftShift".to_owned(),
    RightShift => "RightShift".to_owned(),
    DoubleAmper => "DoubleAmper".to_owned(),
    DoubleVbar => "DoubleVbar".to_owned(),
    Exclamation => "Exclamation".to_owned(),
    Greater => "Greater".to_owned(),
    Less => "Less".to_owned(),
    Equal => "Equal".to_owned(),
    DoubleEqual => "DoubleEqual".to_owned(),
    LPar => "LPar".to_owned(),
    RPar => "RPar".to_owned(),
    LBracket => "LBracket".to_owned(),
    RBracket => "RBracket".to_owned(),
    LBrace => "LBrace".to_owned(),
    RBrace => "RBrace".to_owned(),
    Dot => "Dot".to_owned(),
    Semicolon => "Semicolon".to_owned(),
    Comma => "Comma".to_owned(),
    Function => "FUNCTION".to_owned(),
    Program => "MAIN".to_owned(),
    Not => "Not".to_owned(),
    And => "And".to_owned(),
    Or => "Or".to_owned()
  }
}

pub fn is_same_token_kind(token: Token, other: Token) -> bool {
  get_token_kind(token) == get_token_kind(other)
}

#[cfg(test)]
mod tests {
  use super::{get_token_kind, is_same_token_kind, Token};

  #[test]
  fn test_token_eq() {
    assert_eq!(
      "Id",
      get_token_kind(Token::Id {
        name: "zhansan".to_owned()
      })
    );

    assert_eq!(
      true,
      is_same_token_kind(
        Token::Id {
          name: "zhansan".to_owned()
        },
        Token::Id {
          name: "Lisi".to_owned()
        }
      )
    );
  }
}
