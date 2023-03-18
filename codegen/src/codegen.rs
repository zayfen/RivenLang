use parser::ast::{
  ArithmeticExpr, AssignStmt, CallExpr, CompareExpr, CompareOp, ComponentArithmeticExpr,
  ComponentFactor, ComponentFactorValue, ComponentTerm, Expression, ExpressionValue, Factor,
  FactorValue, FunctionStmt, Identifier, IfStmt, LogicExpr, LogicOp, Primary, PrimaryValue,
  Program, ReturnStmt, Statement, StatementValue, StmtList, Term,
};

/// code generator
///
///

#[derive(Debug)]
pub struct Emitter {
  headers: Vec<String>,
  lines: Vec<String>,
  tail: Vec<String>,
}

impl Emitter {
  pub fn new() -> Self {
    Emitter {
      headers: vec![],
      lines: vec![],
      tail: vec![],
    }
  }

  pub fn push_header(&mut self, code: &str) {
    self.headers.push(code.into());
  }

  pub fn emmit(&mut self, code: &str) {
    self.lines.push(code.into());
  }

  pub fn push_tail(&mut self, code: &str) {
    self.tail.push(code.into());
  }

  pub fn gen_code(&self) -> String {
    let source_code_header = self.headers.join("\n");
    let souce_code_body = self.lines.join(" ");
    let source_code_tail = self.tail.join("\n");

    format!(
      "{}\n{}\n{}",
      source_code_header, souce_code_body, source_code_tail
    )
  }

  // write generate source code to file
  pub(crate) fn write_file(&self) {}
}

pub trait CodeGenerator {
  fn visit_identifier(&mut self, identifier: &Identifier);
  fn visit_primary(&mut self, primary: &Primary);
  fn visit_factor(&mut self, factor: &Factor);
  fn visit_term(&mut self, term: &Term);
  fn visit_arithmetic_expr(&mut self, arithmetic_expr: &ArithmeticExpr);
  fn visit_component_factor(&mut self, factor: &ComponentFactor);
  fn visit_component_term(&mut self, term: &ComponentTerm);
  fn visit_component_arithmetic_expr(&mut self, arith_expr: &ComponentArithmeticExpr);
  fn visit_call_expr(&mut self, call_expr: &CallExpr);
  fn visit_call_stmt(&mut self, call_expr: &CallExpr);
  fn visit_expr(&mut self, expr: &Expression);
  fn visit_compare_expr(&mut self, compare_expr: &CompareExpr);
  fn visit_logic_expr(&mut self, logic_expr: &LogicExpr);
  fn visit_assign_stmt(&mut self, stmt: &AssignStmt);
  fn visit_return_stmt(&mut self, stmt: &ReturnStmt);
  fn visit_if_stmt(&mut self, stmt: &IfStmt);
  fn visit_function_stmt(&mut self, stmt: &FunctionStmt);
  fn visit_stmt(&mut self, stmt: &Statement);
  fn visit_stmt_list(&mut self, stmt_list: &StmtList);
  fn visit_program(&mut self, program: &Program);
}

pub struct CCodeGenManager<'a> {
  emitter: &'a mut Emitter,
}

impl<'a> CCodeGenManager<'a> {
  pub fn new(emitter: &'a mut Emitter) -> Self {
    emitter.push_header("#include<stdio.h>");
    emitter.push_header("#include<stdlib.h>");

    CCodeGenManager { emitter }
  }
}

impl<'a> CodeGenerator for CCodeGenManager<'a> {
  fn visit_primary(&mut self, primary: &Primary) {
    match primary {
      Primary(PrimaryValue::String(s)) => self.emitter.emmit(format!("{:?}", s).as_str()),
      Primary(PrimaryValue::Number(n)) => self.emitter.emmit(format!("{}", n).as_str()),
    }
  }

  fn visit_identifier(&mut self, identifier: &Identifier) {
    let id = identifier.to_string();
    self.emitter.emmit(id.as_str());
  }

  fn visit_factor(&mut self, factor: &Factor) {
    match factor {
      Factor(FactorValue::Primary(primary)) => self.visit_primary(primary),
      Factor(FactorValue::Identifier(identifier)) => self.visit_identifier(identifier),
      Factor(FactorValue::CallExpr(call_expr)) => self.visit_call_expr(call_expr),
    }
  }

  fn visit_term(&mut self, term: &Term) {
    self.visit_factor(&term.0);

    let opt_bin_op = &term.1;
    let opt_term = &term.2;
    if let Some(bin_op) = opt_bin_op {
      self.emitter.emmit(bin_op.to_string().as_str());
    }

    if let Some(_term) = opt_term {
      self.visit_term(_term);
    }
  }

  fn visit_arithmetic_expr(&mut self, arithmetic_expr: &ArithmeticExpr) {
    self.visit_term(&arithmetic_expr.0);
    let opt_bin_op = &arithmetic_expr.1;
    let opt_box_arithmetic_expr = &arithmetic_expr.2;

    if let Some(bin_op) = opt_bin_op {
      self.emitter.emmit(bin_op.to_string().as_str());
    }

    if let Some(arit_expr) = opt_box_arithmetic_expr {
      self.visit_arithmetic_expr(arit_expr);
    }
  }

  fn visit_call_expr(&mut self, call_expr: &CallExpr) {
    self.visit_identifier(&call_expr.0);
    self.emitter.emmit("(");
    let args = &call_expr.1 .0;
    args.iter().enumerate().for_each(|(idx, id)| {
      self.visit_expr(id);
      // for last identifier, dont emmit ","
      if idx < (args.len() - 1) {
        self.emitter.emmit(",");
      }
    });

    self.emitter.emmit(")");
  }

  fn visit_call_stmt(&mut self, call_expr: &CallExpr) {
    self.visit_call_expr(call_expr);
    self.emitter.emmit(";");
  }

  fn visit_expr(&mut self, expr: &Expression) {
    let expr_value = &expr.0;
    match expr_value {
      ExpressionValue::ComponentArithmeticExpr(arith_expr) => self.visit_component_arithmetic_expr(arith_expr),
    }
  }

  fn visit_compare_expr(&mut self, compare_expr: &CompareExpr) {
    self.visit_expr(&compare_expr.0);
    match compare_expr.1 {
      Some(CompareOp::Eq) => self.emitter.emmit("=="),
      Some(CompareOp::Gt) => self.emitter.emmit(">"),
      Some(CompareOp::Lt) => self.emitter.emmit("<"),
      None => (),
    };

    if let Some(expr) = &compare_expr.2 {
      self.visit_expr(expr);
    }
  }

  fn visit_logic_expr(&mut self, logic_expr: &LogicExpr) {
    match logic_expr.0 {
      LogicOp::Bool => {
        if let Some(compare_expr) = &logic_expr.1 {
          self.visit_compare_expr(compare_expr);
        }
      }
      LogicOp::And => {
        self.emitter.emmit("(");
        if let Some(expr) = &logic_expr.2 {
          self.visit_logic_expr(expr);
        }

        if logic_expr.2.is_some() && logic_expr.3.is_some() {
          self.emitter.emmit("&&");
        }

        if let Some(expr) = &logic_expr.3 {
          self.visit_logic_expr(expr);
        }

        self.emitter.emmit(")");
      }

      LogicOp::Or => {
        self.emitter.emmit("(");
        if let Some(expr) = &logic_expr.2 {
          self.visit_logic_expr(expr);
        }

        if logic_expr.2.is_some() && logic_expr.3.is_some() {
          self.emitter.emmit("||");
        }

        if let Some(expr) = &logic_expr.3 {
          self.visit_logic_expr(expr);
        }

        self.emitter.emmit(")");
      }
      LogicOp::Not => {
        if logic_expr.2.is_some() {
          self.emitter.emmit("!");
          if let Some(expr) = &logic_expr.2 {
            self.visit_logic_expr(expr);
          }
        }
      }
    }
  }

  fn visit_assign_stmt(&mut self, stmt: &AssignStmt) {
    // TODO: check variable type here
    // assume it's int type
    self.emitter.emmit("int");

    self.visit_identifier(&stmt.0);
    self.emitter.emmit("=");
    self.visit_expr(&stmt.1);
    self.emitter.emmit(";");
  }

  fn visit_return_stmt(&mut self, stmt: &ReturnStmt) {
    self.emitter.emmit("return");
    self.visit_expr(&stmt.0);
    self.emitter.emmit(";");
  }

  fn visit_if_stmt(&mut self, stmt: &IfStmt) {
    self.emitter.emmit("if (");
    self.visit_logic_expr(&stmt.0);
    self.emitter.emmit(") {");

    if let Some(stmt_list) = &stmt.1 {
      self.visit_stmt_list(&stmt_list);
    }

    self.emitter.emmit("}");
  }

  fn visit_function_stmt(&mut self, stmt: &FunctionStmt) {
    // TODO: check function return type, assume int type here
    self.emitter.emmit("int");
    self.visit_identifier(&stmt.0);
    self.emitter.emmit("(");
    stmt.1.iter().enumerate().for_each(|(idx, id)| {
      // FIXME: assume int type
      self.emitter.emmit("int");

      self.visit_identifier(id);
      if idx < (stmt.1.len() - 1) {
        self.emitter.emmit(",");
      }
    });
    self.emitter.emmit(") {");
    if let Some(stmt_list) = &stmt.2 {
      self.visit_stmt_list(stmt_list);
    }
    self.emitter.emmit("}");
  }

  fn visit_stmt(&mut self, stmt: &Statement) {
    match &stmt.0 {
      StatementValue::AssignStmt(stmt) => self.visit_assign_stmt(stmt),
      StatementValue::CallStmt(stmt) => self.visit_call_stmt(stmt),
      StatementValue::FunctionStmt(stmt) => self.visit_function_stmt(stmt),
      StatementValue::IfStmt(stmt) => self.visit_if_stmt(stmt),
      StatementValue::ReturnStmt(stmt) => self.visit_return_stmt(stmt),
    }
  }

  fn visit_stmt_list(&mut self, stmt_list: &StmtList) {
    if let Some(stmt) = &stmt_list.0 {
      self.visit_stmt(stmt);
    }

    if let Some(rests_stmts) = &stmt_list.1 {
      self.visit_stmt_list(rests_stmts);
    }
  }

  fn visit_program(&mut self, program: &Program) {
    self.visit_stmt_list(&program.0);
  }

  fn visit_component_factor(&mut self, factor: &ComponentFactor) {
    let in_parentheses = factor.0;
    if in_parentheses {
      self.emitter.emmit("(");
    }
    match &factor.1 {
      ComponentFactorValue::ArithmeticExpr(expr) => self.visit_arithmetic_expr(expr),
      ComponentFactorValue::ComponentFactor(Some(factor)) => {
        self.visit_component_factor(&factor);
      },
      ComponentFactorValue::ComponentFactor(None) => ()
    }
    if in_parentheses {
      self.emitter.emmit(")");
    }
  }

  fn visit_component_term(&mut self, term: &ComponentTerm) {
    self.visit_component_factor(&term.0);

    let opt_bin_op = &term.1;
    let opt_term = &term.2;
    if let Some(bin_op) = opt_bin_op {
      self.emitter.emmit(bin_op.to_string().as_str());
    }

    if let Some(_term) = opt_term {
      self.visit_component_term(_term);
    }
  }

  fn visit_component_arithmetic_expr(&mut self, arithmetic_expr: &ComponentArithmeticExpr) {
    self.visit_component_term(&arithmetic_expr.0);
    let opt_bin_op = &arithmetic_expr.1;
    let opt_box_arithmetic_expr = &arithmetic_expr.2;

    if let Some(bin_op) = opt_bin_op {
      self.emitter.emmit(bin_op.to_string().as_str());
    }

    if let Some(arit_expr) = opt_box_arithmetic_expr {
      self.visit_component_arithmetic_expr(arit_expr);
    }
  }
}
