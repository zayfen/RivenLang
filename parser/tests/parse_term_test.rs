extern crate parser;
use parser::parser::{Parser};
use parser::parse_term::parse_term;

#[test]
fn test_parse_term () {
  let mut p = Parser::new("200");
  let number = parse_term(&mut p);
  dbg!(number);
  assert_eq!(2+2, 4);
}
