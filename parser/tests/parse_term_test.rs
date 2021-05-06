use Parser;
#[test]
fn test_parse_term () {
  let mut p = Parser::new("200");
  let number = parse_term(&mut p);
  assert_eq!(2+2, 4);
}
