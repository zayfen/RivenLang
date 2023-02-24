use log::debug;
use parser::{
  ast::{Primary, PrimaryValue},
  parse_primary::parse_primary,
  parser::Parser,
};

#[test]
fn test_parse_primary_string() {
  let mut parser = Parser::new("'123'");
  let token = parser.get_token();
  let primary = parse_primary(&mut parser);

  assert_eq!(
    primary,
    Primary::from(PrimaryValue::String("123".to_owned()))
  );

  println!("{:?}", primary);
}

#[test]
fn test_parse_primary_number() {
  let mut parser = Parser::new("123");
  let token = parser.get_token();
  let primary = parse_primary(&mut parser);

  assert_eq!(primary, Primary::from(PrimaryValue::Number(123f64)));
  println!("{:?}", primary);
}
