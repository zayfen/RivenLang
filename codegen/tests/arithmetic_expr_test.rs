use codegen::codegen::{CCodeGenManager, CodeGenerator, Emitter};
use parser::{
  parse_arithmetic_expr::parse_arithmetic_expr, parse_call_expr::parse_call_expr, parser::Parser,
};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_arithmetic_expr_codegen() {
    let mut p = Parser::new("number + 1 * 2 / 3 - 100");
    let mut emitter = Emitter::new();
    let mut codegen = CCodeGenManager::new(&mut emitter);
    codegen.visit_arithmetic_expr(&parse_arithmetic_expr(&mut p));

    println!("{}", emitter.gen_code());

    assert_eq!(
      emitter.gen_code().find("number + 1 * 2 / 3 - 100"),
      Some(54)
    );
  }

  #[test]
  fn test_call_expr_codegen() {
    let mut p = Parser::new("printf(fmt, name, age)");
    let mut emitter = Emitter::new();
    let mut codegen = CCodeGenManager::new(&mut emitter);
    codegen.visit_call_expr(&parse_call_expr(&mut p));

    println!("{}", emitter.gen_code());

    assert_eq!(
      emitter.gen_code().find("number + 1 * 2 / 3 - 100"),
      Some(54)
    );
  }
}
