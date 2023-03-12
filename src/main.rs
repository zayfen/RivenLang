use parser::{
  parse_primary::parse_primary,
  parser::Parser,
};
use codegen::codegen::{CCodeGenManager, CodeGenerator, Emitter};

fn main() {
  let mut p = Parser::new("'hello world'");
  let primary = parse_primary(&mut p);

  let mut emmiter = Emitter::new();
  let mut codegen = CCodeGenManager::new(&mut emmiter);

  codegen.visit_primary(&primary);
  println!("Generated source code: ");
  println!("{}", emmiter.gen_code());
}
