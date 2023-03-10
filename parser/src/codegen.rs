/// code generator
///
///

#[derive(Debug)]
pub struct Emitter<'a> {
  headers: Vec<&'a str>,
  lines: Vec<&'a str>,
  tail: Vec<&'a str>,
}

impl<'a> Emitter<'a> {
  fn new() -> Self {
    Emitter {
      headers: vec![],
      lines: vec![],
      tail: vec![],
    }
  }

  fn push_header(&mut self, code: &'a str) {
    self.headers.push(code);
  }

  fn push_line(&mut self, code: &'a str) {}

  fn push_tail(&mut self, code: &'a str) {}

  fn gen_code(&self) -> String {
    String::from("")
  }

  // write generate source code to file
  fn write_file(&self) {}
}
