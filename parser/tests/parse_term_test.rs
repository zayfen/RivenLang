use parser::{
  ast::{Factor, FactorValue, Identifier, Term},
  parse_term::parse_term,
  parser::Parser,
};

#[test]
fn test_parse_term() {
  let mut parser = Parser::new("name * 3 / 2 * count");
  let token = parser.get_token();
  let term = parse_term(&mut parser);

  assert_eq!(
    term.0,
    Factor::from(FactorValue::Identifier(Identifier::from("name")))
  );
  println!("{:?}", term);
}
