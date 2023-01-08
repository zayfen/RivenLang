extern crate parser;
use parser::parse_literal::parse_literal;
use parser::parser::Parser;

#[test]
fn test_parse_literal() {
  let mut p = Parser::new("200");
  let number = parse_literal(&mut p);
  dbg!(number);
  assert_eq!(2 + 2, 4);
}
