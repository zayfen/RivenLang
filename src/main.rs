use parser::{
  parser::Parser, parse_program::parse_program,
};
use codegen::codegen::{CCodeGenManager, CodeGenerator, Emitter};

fn main() {
  let mut p = Parser::new("program {
    function fib(n) {
      printf(\"%l\", n);
      nn = n-1;
      if (nn) {
        return fib(nn);
      }
    }

    function main() {
      n = 10;
      fib(n);
      return 0;
    }
  }");

  let program = parse_program(&mut p);

  let mut emmiter = Emitter::new();
  let mut codegen = CCodeGenManager::new(&mut emmiter);

  codegen.visit_program(&program);
  println!("Generated source code: ");
  println!("{}", emmiter.gen_code());
}
