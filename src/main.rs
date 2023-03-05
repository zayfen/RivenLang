use parser::{parser::Parser, parse_arithmetic_expr::parse_arithmetic_expr};

fn main() {
  let mut p = Parser::new("name + 3 / 2 * count");
  println!("{:?}", p);
  let expr = parse_arithmetic_expr(&mut p);
  println!("expr: {:?}", expr);
}
