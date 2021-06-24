// virtual node for zlang
use crate::location::SourceLocation;

// 所有的Node枚举
#[derive(Copy, Clone, Debug)]
pub enum Kind {
  Program,
  Function,
  EmptyStatement,
  BlockStatement,
  ExpressionStatement,
  IfStatement,
  ReturnStatement,
  WhileStatement,
  ForInStatement,
  ArrayExpression,
  AssignmentExpression,
  LogicalExpression,
  CallExpression,
  BinaryExpression,
  AssignmentOperator,
  Identifier,
  Literal,
  LogicalOperator,
  Property,
  UnaryOperator,
  BinaryOperator,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
  Assign,
  Plus,
  Minus,
  Time,
  Div,
  Mode,
  LShift,
  RShift,
  Or,
  Xor,
  And,
}

// Literals, including i64, f64, string

#[derive(Debug, Clone)]
pub enum LiteralValue {
  Integer(i64),
  Float(f64),
  Str(String),
}

#[derive(Debug, Clone)]
pub struct Literal {
  pub kind: Kind,
  pub value: LiteralValue,
}

impl Literal {
  pub fn new(value: LiteralValue) -> Self {
    Literal {
      kind: Kind::Literal,
      value: value,
    }
  }
}

// Identifier
pub struct Identifier {
  pub kind: Kind,
  value: String,
}

impl Identifier {
  pub fn new(name: &str) -> Self {
    Identifier {
      kind: Kind::Identifier,
      value: String::from(name),
    }
  }
}

// expressions, including BinaryExpression, CallExpressioin, LogicalExpression

pub trait Expression {
  fn new() -> Self
  where
    Self: Sized;
  fn get_kind(&self) -> Kind;
}

pub struct AssignExpr {
  pub kind: Kind,
  pub left: Identifier,
  pub operator: BinaryOperator,
  pub right: Option<Box<dyn Expression>>,
}

impl Expression for AssignExpr {
  fn new() -> Self {
    AssignExpr {
      kind: Kind::AssignmentExpression,
      left: Identifier::new(""),
      operator: BinaryOperator::Assign,
      right: None,
    }
  }

  fn get_kind(&self) -> Kind {
    self.kind.clone()
  }
}

// statements, including Function statement, BlockStatement, IfStatement, WhileStatement, ForStatement
// AssignStatement

// Program
