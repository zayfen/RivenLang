extern crate parser;
use parser::ast::*;
use parser::parse_term::parse_term;
use parser::parser::Parser;

#[test]
fn test_parse_term() {
  let code = "200";
  // let code = "0b1000000000000000000000000000000010000000000000000000000000000001";
  let mut p = Parser::new(code);
  let number = parse_term(&mut p);
  match number.unwrap().value {
    LiteralValue::Integer(integer) => assert_eq!(integer, 200),
    _ => assert_eq!(-1, 0),
  }

  let code = "\"String\"";
  let mut p = Parser::new(code);
  let str = parse_term(&mut p);
  match str.unwrap().value {
    LiteralValue::Str(s) => assert_eq!(s, "String"),
    _ => assert_eq!(-1, 0),
  }

  let code = "1.20";
  let mut p = Parser::new(code);
  let float = parse_term(&mut p);
  match float.unwrap().value {
    LiteralValue::Float(f) => assert_eq!(f, 1.2),
    _ => assert_eq!(-1, 0),
  }
}
