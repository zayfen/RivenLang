use std::{
  env,
  fmt::Display,
  fs::File,
  io::{self, BufRead, Write, Read},
  path::Path,
  process::{Command, Stdio},
};

use codegen::codegen::{CCodeGenManager, CodeGenerator, Emitter};
use parser::{parse_program::parse_program, parser::Parser};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("missing source file");
  }

  let source_file = args.get(1);
  let source_file_path = Path::new(source_file.unwrap());
  let result_lines = read_lines(source_file_path);
  let mut source = String::from("");
  if let Ok(lines) = result_lines {
    let _lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    source = _lines.join("\n");
  }

  let mut p = Parser::new(&source);
  let program = parse_program(&mut p);

  let mut emmiter = Emitter::new();
  let mut codegen = CCodeGenManager::new(&mut emmiter);
  codegen.visit_program(&program);

  let c_source_path = source_file_path.canonicalize().unwrap().with_extension("c");
  let display = c_source_path.display();

  let mut c_file = match File::create(&c_source_path) {
    Err(why) => panic!("couldn't create {}: {}", display, why),
    Ok(file) => file,
  };

  match c_file.write_all(emmiter.gen_code().as_bytes()) {
    Err(why) => panic!("couldn't write to {}: {}", display, why),
    Ok(_) => println!("successfully wrote to {}", display),
  }

  match c_file.sync_all() {
    Err(why) => panic!("couldn't sync all to c source file {}: {}", display, why),
    Ok(_) => println!("successfully sync c source file({}) to disk", display)
  }

  // Spawn the `gcc` command
  println!("building c source code...");
  let process = match Command::new("gcc")
    .args(&["-Wall", display.to_string().as_str(), "-o", "b.out"])
    .stdout(Stdio::piped())
    .spawn()
  {
    Err(why) => panic!("couldn't spawn gcc: {} source file {}", why, display),
    Ok(process) => process,
  };

  // Because `stdin` does not live after the above calls, it is `drop`ed,
  // and the pipe is closed.
  //
  // This is very important, otherwise `gcc` wouldn't start processing the
  // input we just sent.

  // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
  let mut s = String::new();
  match process.stdout.unwrap().read_to_string(&mut s) {
    Err(why) => panic!("couldn't read gcc stdout: {}", why),
    Ok(_) => print!("Build completed:\n{}", s),
  }
}
