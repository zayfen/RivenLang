// virtual node for zlang
use crate::location::SourceLocation;

// 所有的Node枚举
#[derive(Clone, Debug)]
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

// Literals, including i64, f64, string

#[derive(Debug, Clone)]
pub enum LiteralValue {
  Integer(i64),
  Float(f64),
  Str(String)
}

#[derive(Debug, Clone)]
pub struct Literal {
  pub kind: Kind,
  pub value: LiteralValue
}

impl Literal {
  pub fn new (value: LiteralValue) -> Self {
    Literal {
      kind: Kind::Literal,
      value: value
    }
  }
}


// expressions, including BinaryExpression, CallExpressioin, LogicalExpression


// statements, including Function statement, BlockStatement, IfStatement, WhileStatement, ForStatement
// AssignStatement

// Program
