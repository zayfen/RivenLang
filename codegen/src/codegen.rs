use parser::ast::{Factor, FactorValue, Identifier, Primary, PrimaryValue};

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

  pub fn push_line(&mut self, code: &str) {
    self.lines.push(code.into());
  }

  pub fn push_tail(&mut self, code: &str) {
    self.tail.push(code.into());
  }

  pub fn gen_code(&self) -> String {
    let source_code_header = self.headers.join("\n");
    let souce_code_body = self.lines.join("\n");
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
  fn visit_primary(&mut self, primary: &Primary);
  fn visit_identifier(&mut self, identifier: &Identifier);
  fn visit_factor(&mut self, factor: &Factor);
}

pub struct CCodeGenManager<'a> {
  emitter: &'a mut Emitter,
}

impl<'a> CCodeGenManager<'a> {
  pub fn new(emitter: &'a mut Emitter) -> Self {
    emitter.push_header("#include<stdio.h>");
    emitter.push_header("#include<stdlib.h>");
    emitter.push_header("int main(void) {");

    emitter.push_tail("return 0;");
    emitter.push_tail("}");
    CCodeGenManager { emitter }
  }
}

impl<'a> CodeGenerator for CCodeGenManager<'a> {
  fn visit_primary(&mut self, primary: &Primary) {
    match primary {
      Primary(PrimaryValue::String(s)) => self.emitter.push_line(format!("{:?}", s).as_str()),
      Primary(PrimaryValue::Number(n)) => self.emitter.push_line(format!("{:?}", n).as_str()),
    }
  }

  fn visit_identifier(&mut self, identifier: &Identifier) {
    let id = identifier.to_string();
    self.emitter.push_line(id.as_str());
  }

  fn visit_factor(&mut self, factor: &Factor) {
    match factor {
      Factor(FactorValue::Primary(primary)) => self.visit_primary(primary),
      Factor(FactorValue::Identifier(identifier)) => self.visit_identifier(identifier),
    }
  }
}
