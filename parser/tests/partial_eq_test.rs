extern crate parser;
use parser::{ast::Kind, token::Token};

#[test]
fn test_token_partial_eq() {
  assert_eq!(
    Token::Id {
      name: "123".to_owned()
    },
    Token::Id {
      name: "123".to_owned()
    }
  )
}

#[test]
fn test_ast_kind() {
  assert_eq!(Kind::Program, Kind::Program);
}
