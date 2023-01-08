use num_bigint::BigInt;
use parser::{
  ast::{EnumBinaryOperators, Literal, Primary, PrimaryValue, Term},
  parse_term::parse_term,
  parser::Parser,
  token::{is_same_token_kind, NumberType, Token},
};

#[test]
fn test_parse_term() {
  let mut parser = Parser::new("123 * 456 / 17");
  let term = parse_term(&mut parser);

  assert_eq!(
    term.clone().unwrap().0,
    Primary::new(PrimaryValue::Constant(Literal::number(123f64)))
  );

  assert_eq!(term.clone().unwrap().1, Some(EnumBinaryOperators::Time));

  let right_term_box = term.clone().unwrap().2.unwrap();

  assert_eq!(
    right_term_box.0,
    Primary::new(PrimaryValue::Constant(Literal::number(456f64)))
  );

  assert_eq!(right_term_box.1, Some(EnumBinaryOperators::Div));

  let right_right_term_box = right_term_box.2.unwrap();
  assert_eq!(
    right_right_term_box.0,
    Primary::new(PrimaryValue::Constant(Literal::number(17f64)))
  );

  assert_eq!(right_right_term_box.1, None);
}

#[test]
fn test_parse_term_2() {
  let mut parser = Parser::new("123+2");
  let term = parse_term(&mut parser).unwrap();

  assert_eq!(
    term.0,
    Primary::new(PrimaryValue::Constant(Literal::number(123f64)))
  );

  assert_eq!(term.1, None);

  assert_eq!(parser.get_token(), Token::Plus);

  assert_eq!(
    parser.peek_token(),
    Token::Number {
      number_type: NumberType::Int,
      int: BigInt::from(2),
      float: 0f64
    }
  );
}
