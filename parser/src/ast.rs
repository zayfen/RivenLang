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
  Primary,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
  kind: Kind,
  loc: SourceLocation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
  kind: Kind,
  body: Vec<Statement>,
}

impl Program {
  pub fn new() -> Self {
    Program {
      kind: Kind::Program,
      body: vec![],
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
  kind: Kind,
  id: Option<Identifier>,
  params: Vec<Property>,
  body: BlockStatement,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier(pub String);

#[derive(Clone, Debug, PartialEq)]
pub struct Property {
  kind: Kind,
  id: Identifier,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
  EmptyStatement(EmptyStatement),
  BlockStatment(BlockStatement),
  ExpressionStatement(ExpressionStatement),
  IfStatement(IfStatement),
  ReturnStatement(ReturnStatement),
  WhileStatement(WhileStatement),
  ForInStatement(ForInStatement),
}

#[derive(Clone, Debug, PartialEq)]
pub struct EmptyStatement {
  kind: Kind,
}

impl EmptyStatement {
  fn new() -> Self {
    EmptyStatement {
      kind: Kind::EmptyStatement,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockStatement {
  kind: Kind,
  body: Option<Vec<Statement>>,
}

impl BlockStatement {
  fn new() -> Self {
    BlockStatement {
      kind: Kind::BlockStatement,
      body: None,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionStatement {
  kind: Kind,
  expression: Option<Expression>,
}

impl ExpressionStatement {
  fn new() -> Self {
    ExpressionStatement {
      kind: Kind::ExpressionStatement,
      expression: None,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfStatement {
  kind: Kind,
  test: Option<Box<Expression>>,
  consequent: Option<Box<Statement>>,
  alternate: Option<Box<Statement>>,
}

impl IfStatement {
  fn new() -> Self {
    IfStatement {
      kind: Kind::IfStatement,
      test: None,
      consequent: None,
      alternate: None,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnStatement {
  kind: Kind,
  argument: Option<Expression>,
}

impl ReturnStatement {
  fn new() -> Self {
    ReturnStatement {
      kind: Kind::ReturnStatement,
      argument: None,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WhileStatement {
  kind: Kind,
  test: Option<Expression>,
  body: Option<BlockStatement>,
}

impl WhileStatement {
  fn new() -> Self {
    WhileStatement {
      kind: Kind::WhileStatement,
      test: None,
      body: None,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForInStatement {
  kind: Kind,
  left: Option<Expression>,
  right: Option<Expression>,
  body: Option<BlockStatement>,
  each: bool,
}

impl ForInStatement {
  fn new() -> Self {
    ForInStatement {
      kind: Kind::ForInStatement,
      left: None,
      right: None,
      body: None,
      each: true,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
  ArrayExpression(Box<ArrayExpression>),
  AssignmentExpression(Box<AssignmentExpression>),
  LogicalExpression(Box<LogicalExpression>),
  CallExpression(Box<CallExpression>),
  BinaryExpression(Box<BinaryExpression>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayExpression {
  kind: Kind,
  elements: Option<Vec<Expression>>,
}

impl ArrayExpression {
  fn new() -> Self {
    ArrayExpression {
      kind: Kind::ArrayExpression,
      elements: None,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignmentExpression {
  kind: Kind,
  operator: Option<AssignmentOperator>,
  left: Option<Expression>,
  right: Option<Expression>,
}

impl AssignmentExpression {
  fn new() -> Self {
    AssignmentExpression {
      kind: Kind::AssignmentExpression,
      operator: None,
      left: None,
      right: None,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LogicalExpression {
  kind: Kind,
  operator: Option<LogicalOperator>,
  left: Option<Expression>,
  right: Option<Expression>,
}

impl LogicalExpression {
  fn new() -> Self {
    LogicalExpression {
      kind: Kind::LogicalExpression,
      operator: None,
      left: None,
      right: None,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CallExpression {
  kind: Kind,
  callee: Option<Box<Expression>>,
  arguments: Option<Vec<Option<Box<Expression>>>>,
}

impl CallExpression {
  fn new() -> Self {
    CallExpression {
      kind: Kind::CallExpression,
      callee: None,
      arguments: None,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExpression {
  kind: Kind,
  operator: Option<BinaryOperator>,
  left: Option<Expression>,
  right: Option<Expression>,
}

impl BinaryExpression {
  fn new() -> Self {
    BinaryExpression {
      kind: Kind::BinaryExpression,
      operator: None,
      left: None,
      right: None,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EnumAssignmentOperators {
  Assign,
  PlusAssign,
  MinusAssign,
  TimeAssign,
  DivAssign,
  ModeAssign,
  LShiftAssign,
  RShiftAssign,
  OrAssisn,
  XorAssign,
  AndAssign,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignmentOperator {
  kind: Kind,
  value: EnumAssignmentOperators,
}

impl AssignmentOperator {
  fn new(operator: EnumAssignmentOperators) -> Self {
    AssignmentOperator {
      kind: Kind::AssignmentOperator,
      value: operator,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
enum EnumLogicalOperators {
  LogicalAnd,
  LogicalOr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LogicalOperator {
  kind: Kind,
  value: EnumLogicalOperators,
}

impl LogicalOperator {
  fn new(operator: EnumLogicalOperators) -> Self {
    LogicalOperator {
      kind: Kind::LogicalOperator,
      value: operator,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EnumLiteral {
  String(String),
  Boolean(bool),
  Number(f64),
  None,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Literal {
  pub(crate) kind: Kind,
  pub(crate) value: EnumLiteral,
}

impl Literal {
  pub fn string(s: String) -> Self {
    Literal {
      kind: Kind::Literal,
      value: EnumLiteral::String(s),
    }
  }

  pub fn boolean(b: bool) -> Self {
    Literal {
      kind: Kind::Literal,
      value: EnumLiteral::Boolean(b),
    }
  }

  pub fn number(n: f64) -> Self {
    Literal {
      kind: Kind::Literal,
      value: EnumLiteral::Number(n),
    }
  }

  pub fn none() -> Self {
    Literal {
      kind: Kind::Literal,
      value: EnumLiteral::None,
    }
  }
}

// Primary
#[derive(Clone, Debug, PartialEq)]
pub enum PrimaryValue {
  Constant(Literal),
  Variable(Identifier),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Primary(PrimaryValue);

impl Primary {
  pub fn new(value: PrimaryValue) -> Self {
    Primary(value)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Term(
  pub Primary,
  pub Option<EnumBinaryOperators>,
  pub Option<Box<Term>>,
);

impl Term {
  pub fn new(
    left: Primary,
    operator: Option<EnumBinaryOperators>,
    right: Option<Box<Term>>,
  ) -> Self {
    Term(left, operator, right)
  }
}

// 一元运算符
#[derive(Clone, Debug, PartialEq)]
enum EnumUnaryOperators {
  Not,
  Xor,
  Typeof,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryOperator {
  kind: Kind,
  value: EnumUnaryOperators,
}

impl UnaryOperator {
  fn new(operator: EnumUnaryOperators) -> Self {
    UnaryOperator {
      kind: Kind::UnaryOperator,
      value: operator,
    }
  }
}

// 二元运算符
#[derive(Clone, Debug, PartialEq)]
pub enum EnumBinaryOperators {
  Add,
  Minus,
  Time,
  Div,
  Mode,
  BitwiseAnd,
  BitwiseOr,
  BitwiseXor,
  Equal,
  NotEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
  BitwiseLShift,
  BitwiseRShift,
}

#[derive(Clone, Debug, PartialEq)]
struct BinaryOperator {
  kind: Kind,
  value: EnumBinaryOperators,
}

impl BinaryOperator {
  pub fn new(self, _kind: Kind, _value: EnumBinaryOperators) -> Self {
    BinaryOperator {
      kind: _kind,
      value: _value,
    }
  }
}

// Arithemtic expression
#[derive(Clone, Debug, PartialEq)]
pub struct ArithmeticExpr(
  Term,
  Option<EnumBinaryOperators>,
  Option<Box<ArithmeticExpr>>,
);

impl ArithmeticExpr {
  pub fn new(
    left: Term,
    operator: Option<EnumBinaryOperators>,
    right: Option<Box<ArithmeticExpr>>,
  ) -> Self {
    ArithmeticExpr(left, operator, right)
  }
}
