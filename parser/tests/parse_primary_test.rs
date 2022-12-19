use parser::{
  ast::{Identifier, Literal, Primary, PrimaryValue},
  parse_primary::parse_primary,
  parser::Parser,
};

#[test]
fn test_parse_primary_string() {
  let mut parser = Parser::new("'123'");
  let token = parser.next_token();
  let primary_node_result = parse_primary(&mut parser, token);
  let primary_node: Primary = primary_node_result.unwrap();

  assert_eq!(
    primary_node,
    Primary::new(PrimaryValue::Constant(Literal::string("123".to_owned())))
  );
}

#[test]
fn test_parse_primary_number() {
  let mut parser = Parser::new("123");
  let token = parser.next_token();
  let primary_node_result = parse_primary(&mut parser, token);
  let primary_node = primary_node_result.unwrap();

  assert_eq!(
    primary_node,
    Primary::new(PrimaryValue::Constant(Literal::number(123f64)))
  );
}

#[test]
fn test_parse_primary_id() {
  let mut parser = Parser::new("name");
  let token = parser.next_token();
  let primary_node_result = parse_primary(&mut parser, token);
  let primary_node = primary_node_result.unwrap();

  assert_eq!(
    primary_node,
    Primary::new(PrimaryValue::Variable(Identifier("name".to_owned())))
  )
}
