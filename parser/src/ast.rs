use std::fmt;
use std::fmt::Display;
use std::matches;
use std::string::String;

use crate::token::Token;

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
    matches!(self, PrimaryValue::String(_))
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
  CallExpr(CallExpr),
}

impl FactorValue {
  pub const fn is_primary(&self) -> bool {
    matches!(self, FactorValue::Primary(_))
  }

  pub const fn is_identifier(&self) -> bool {
    matches!(self, FactorValue::Identifier(_))
  }

  pub const fn is_call_expr(&self) -> bool {
    matches!(self, FactorValue::CallExpr(_))
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

impl From<Token> for BinOp {
  fn from(value: Token) -> Self {
    match value {
      Token::Star => BinOp::Time,
      Token::Plus => BinOp::Add,
      Token::Minus => BinOp::Min,
      Token::Slash => BinOp::Div,
      _ => panic!("BinOp can't get from token {:?}", value),
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
pub enum ComponentFactorValue {
  ArithmeticExpr(ArithmeticExpr),
  ComponentFactor(Option<Box<ComponentFactor>>)
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentFactor(pub bool, pub ComponentFactorValue);
impl ComponentFactor {
  pub fn new(in_parentheses: bool, component_factor: ComponentFactorValue) -> Self {
    ComponentFactor(in_parentheses, component_factor)
  }
}

impl From<ComponentFactorValue> for ComponentFactor {
  fn from(value: ComponentFactorValue) -> Self {
    match &value {
      ComponentFactorValue::ArithmeticExpr(_) => ComponentFactor(false, value),
      ComponentFactorValue::ComponentFactor(_) => ComponentFactor(true, value)
    }
  }
}

// BinOp here only can be Time、 Div
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentTerm(
  pub ComponentFactor,
  pub Option<BinOp>,
  pub Option<Box<ComponentTerm>>,
);

impl ComponentTerm {
  pub fn new(
    factor: ComponentFactor,
    op: Option<BinOp>,
    boxed_term: Option<Box<ComponentTerm>>,
  ) -> Self {
    ComponentTerm(factor, op, boxed_term)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentArithmeticExpr(
  pub ComponentTerm,
  pub Option<BinOp>,
  pub Option<Box<ComponentArithmeticExpr>>,
);

impl ComponentArithmeticExpr {
  pub fn new(
    term: ComponentTerm,
    op: Option<BinOp>,
    arith_expr: Option<Box<ComponentArithmeticExpr>>,
  ) -> Self {
    ComponentArithmeticExpr(term, op, arith_expr)
  }
}

impl Display for ComponentArithmeticExpr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "ComponentArithmeticExpr({:?}, {:?}, {:?})",
      self.0,
      self.1,
      self.2.to_owned()
    )
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr(pub Identifier, pub ExpressionList);

impl CallExpr {
  pub fn new(fn_name: Identifier, args: ExpressionList) -> Self {
    CallExpr(fn_name, args)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionValue {
  ComponentArithmeticExpr(ComponentArithmeticExpr)
}

impl ExpressionValue {
  pub fn is_component_arithmetic_expr(&self) -> bool {
    matches!(
      self,
      ExpressionValue::ComponentArithmeticExpr(ComponentArithmeticExpr(_term, _op, _arith_expr))
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
pub struct ExpressionList(pub Vec<Expression>);

#[derive(Debug, Clone, PartialEq)]
pub enum CompareOp {
  Eq, // equal
  Gt, // greater than
  Lt, // less than
}

impl Display for CompareOp {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      CompareOp::Eq => write!(f, "{}", " == "),
      CompareOp::Gt => write!(f, "{}", " > "),
      CompareOp::Lt => write!(f, "{}", " < "),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompareExpr(
  pub Expression,
  pub Option<CompareOp>,
  pub Option<Expression>,
);
impl CompareExpr {
  pub fn new(left: Expression, op: Option<CompareOp>, right: Option<Expression>) -> Self {
    CompareExpr(left, op, right)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicOp {
  Bool, // keep original value, dont transform
  And,
  Or,
  Not,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogicExpr(
  pub LogicOp,
  pub Option<CompareExpr>,
  pub Option<Box<LogicExpr>>,
  pub Option<Box<LogicExpr>>,
);

impl LogicExpr {
  pub fn new(
    op: LogicOp,
    compare_expr: Option<CompareExpr>,
    left_logic_expr: Option<Box<LogicExpr>>,
    right_logic_expr: Option<Box<LogicExpr>>,
  ) -> Self {
    LogicExpr(op, compare_expr, left_logic_expr, right_logic_expr)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicListOp {
  And,
  Not,
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
pub struct IfStmt(pub LogicExpr, pub Option<Box<StmtList>>);

impl IfStmt {
  pub fn new(expr: LogicExpr, stmt_list: StmtList) -> Self {
    IfStmt(expr, Some(Box::new(stmt_list)))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionStmt(
  pub Identifier,
  pub Vec<Identifier>,
  pub Option<Box<StmtList>>,
);

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
