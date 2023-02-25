use parser::{
  ast::{Factor, FactorValue, Identifier, Primary, PrimaryValue},
  parse_factor::parse_factor,
  parser::Parser,
};

#[test]
fn test_parse_factor_primary() {
  let mut parser = Parser::new("'123'");
  let token = parser.get_token();
  let factor = parse_factor(&mut parser);

  assert_eq!(
    factor,
    Factor::from(FactorValue::Primary(Primary::from(PrimaryValue::String(
      "123".to_owned()
    ))))
  );

  println!("{:?}", factor);
}

#[test]
fn test_parse_primary_id() {
  let mut parser = Parser::new("identifier");
  let token = parser.get_token();
  let factor = parse_factor(&mut parser);

  assert_eq!(
    factor,
    Factor::from(FactorValue::Identifier(Identifier::from("identifier")))
  );
  println!("{:?}", factor);
}
