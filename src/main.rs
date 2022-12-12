use parser::parser::Parser;

fn main() {
  let p = Parser::new("a = 10");
  println!("{:?}", p);
  println!("Hello, world!");
  println!("Hello");
}
