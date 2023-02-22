use std::fmt;
use std::fmt::Display;
use std::matches;
use std::string::String;

/*
 * virtual node for zlang
*/
use crate::location::SourceLocation;

// 所有的Node枚举
#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
  Program,
  Function,
  EmptyStatement,
  BlockStatement,
  ExpressionList,
  Expression,
  CallExpression,
  AssignExpression,
  IfStatement,
  ReturnStatement,
  WhileStatement,
  Primary,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier(String);
impl Identifier {
  pub fn new(id: &str) -> Identifier {
    Identifier(id.to_owned())
  }
}

impl Display for Identifier {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Identifier({:?})", self.0)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimaryValue {
  String(String),
  Number(f64),
}

impl PrimaryValue {
  pub const fn is_string(&self) -> bool {
    matches!(*self, PrimaryValue::String(_))
  }

  pub const fn is_number(&self) -> bool {
    !self.is_string()
  }
}

impl Display for PrimaryValue {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      PrimaryValue::String(s) => write!(f, "{}", s),
      PrimaryValue::Number(n) => write!(f, "{}", n),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Primary(PrimaryValue);

impl From<PrimaryValue> for Primary {
  fn from(item: PrimaryValue) -> Self {
    Primary(item)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FactorValue {
  Primary(Primary),
  Identifier(Identifier),
}

impl FactorValue {
  pub const fn is_primary(&self) -> bool {
    matches!(*self, FactorValue::Primary(_))
  }

  pub const fn is_identifier(&self) -> bool {
    matches!(*self, FactorValue::Identifier(_))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Factor(FactorValue);

impl From<FactorValue> for Factor {
  fn from(item: FactorValue) -> Self {
    Factor(item)
  }
}
