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
    write!(f, "{}", self.0)
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
pub struct Primary(pub PrimaryValue);

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
pub struct Factor(pub FactorValue);

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

impl Display for BinOp {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      BinOp::Time => write!(f, "{}", " * "),
      BinOp::Div => write!(f, "{}", " / "),
      BinOp::Add => write!(f, "{}", " + "),
      BinOp::Min => write!(f, "{}", " - "),
    }
  }
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
pub struct Term(pub Factor, pub Option<BinOp>, pub Option<Box<Term>>);

impl Term {
  pub fn new(factor: Factor, op: Option<BinOp>, boxed_term: Option<Box<Term>>) -> Self {
    Term(factor, op, boxed_term)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArithmeticExpr(pub Term, pub Option<BinOp>, pub Option<Box<ArithmeticExpr>>);

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
pub struct CallExpr(pub Identifier, pub Vec<Identifier>);

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
pub struct Expression(pub ExpressionValue);

impl From<ExpressionValue> for Expression {
  fn from(value: ExpressionValue) -> Self {
    Expression(value)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignStmt(pub Identifier, pub Expression);

impl AssignStmt {
  pub fn new(id: Identifier, expr: Expression) -> Self {
    AssignStmt(id, expr)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt(pub Expression);

impl ReturnStmt {
  pub fn new(expr: Expression) -> Self {
    ReturnStmt(expr)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt(pub Expression, pub Option<Box<StmtList>>);

impl IfStmt {
  pub fn new(expr: Expression, stmt_list: StmtList) -> Self {
    IfStmt(expr, Some(Box::new(stmt_list)))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionStmt(pub Identifier, pub Vec<Identifier>, pub Option<Box<StmtList>>);

impl FunctionStmt {
  pub fn new(id: Identifier, params: Vec<Identifier>, stmt_list: StmtList) -> Self {
    FunctionStmt(id, params, Some(Box::new(stmt_list)))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementValue {
  AssignStmt(AssignStmt),
  CallStmt(CallExpr),
  ReturnStmt(ReturnStmt),
  FunctionStmt(FunctionStmt),
  IfStmt(IfStmt),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statement(pub StatementValue);

impl From<StatementValue> for Statement {
  fn from(value: StatementValue) -> Self {
    Statement(value)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StmtList(pub Option<Statement>, pub Option<Box<StmtList>>);

#[derive(Debug, Clone, PartialEq)]
pub struct Program(pub StmtList);
