extern crate parser;
use parser::parse_term::parse_term;
use parser::parser::Parser;
use parser::ast::*;

#[test]
fn test_parse_term() {
  let code = "200";
  // let code = "0b1000000000000000000000000000000010000000000000000000000000000001";
  let mut p = Parser::new(code);
  let number = parse_term(&mut p);
  match number.unwrap().value {
    LiteralValue::Integer(integer) => assert_eq!(integer, 200),
    _ => assert_eq!(200, 0)
  }


  let code = "'String'";
  let mut p = Parser::new(code);
  let str = parse_term(&mut p);
  match str.unwrap().value {
    LiteralValue::Str(s) => assert_eq!(s, "String"),
    _ => assert_eq!(200, 0)
  }
}
