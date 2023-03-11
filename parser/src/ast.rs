use std::fmt;
use std::fmt::Display;
use std::matches;
use std::string::String;

// use crate::location::SourceLocation;

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
impl From<&str> for Identifier {
  fn from(item: &str) -> Self {
    Identifier(item.to_owned())
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
pub struct Primary(pub(crate) PrimaryValue);

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

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
  Time,
  Div,
  Add,
  Min,
}

impl BinOp {
  pub fn is_time(&self) -> bool {
    matches!(*self, BinOp::Time)
  }

  pub fn is_div(&self) -> bool {
    matches!(*self, BinOp::Div)
  }

  pub fn is_add(&self) -> bool {
    matches!(*self, BinOp::Add)
  }

  pub fn is_min(&self) -> bool {
    matches!(*self, BinOp::Min)
  }
}

// BinOp here only can be Time、 Div
#[derive(Debug, Clone, PartialEq)]
pub struct Term(pub Factor, Option<BinOp>, Option<Box<Term>>);

impl Term {
  pub fn new(factor: Factor, op: Option<BinOp>, boxed_term: Option<Box<Term>>) -> Self {
    Term(factor, op, boxed_term)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArithmeticExpr(pub Term, Option<BinOp>, Option<Box<ArithmeticExpr>>);

impl ArithmeticExpr {
  pub fn new(term: Term, op: Option<BinOp>, arith_expr: Option<Box<ArithmeticExpr>>) -> Self {
    ArithmeticExpr(term, op, arith_expr)
  }
}

impl Display for ArithmeticExpr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "ArithmeticExpr({:?}, {:?}, {:?})",
      self.0,
      self.1,
      self.2.to_owned()
    )
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr(Identifier, Vec<Identifier>);

impl CallExpr {
  pub fn new(fn_name: Identifier, args: Vec<Identifier>) -> Self {
    CallExpr(fn_name, args)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionValue {
  CallExpr(CallExpr),
  ArithmeticExpr(ArithmeticExpr),
}

impl ExpressionValue {
  pub fn is_call_expr(&self) -> bool {
    matches!(self, ExpressionValue::CallExpr(CallExpr(id, args)))
  }

  pub fn is_arithmetic_expr(&self) -> bool {
    matches!(
      self,
      ExpressionValue::ArithmeticExpr(ArithmeticExpr(term, op, arith_expr))
    )
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression(ExpressionValue);

impl From<ExpressionValue> for Expression {
  fn from(value: ExpressionValue) -> Self {
    Expression(value)
  }
}
