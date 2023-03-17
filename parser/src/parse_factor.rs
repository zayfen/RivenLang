use crate::ast::{Factor, FactorValue, Identifier};
use crate::parser::Parser;
use crate::token::Token;
use crate::{
  parse_call_expr::parse_call_expr,
  parse_primary::{match_primary, parse_primary},
};

pub(crate) fn match_factor(parser: &mut Parser) -> bool {
  let token = parser.get_token();
  // when token matched Identifier, next_token should be +-x/ (, can't be assign statement
  match_primary(token.clone()) || matches!(token, Token::Id { name: _ })
}

pub fn parse_factor(parser: &mut Parser) -> Factor {
  let token = parser.get_token();
  match token {
    Token::Id { name } => {
      // is call expression
      if parser.peek_token().is_lpar() {
        return Factor::from(FactorValue::CallExpr(parse_call_expr(parser)));
      }

      // identifier
      parser.advance_token();
      return Factor::from(FactorValue::Identifier(Identifier::from(name.as_str())));
    }
    _ => Factor::from(FactorValue::Primary(parse_primary(parser))),
  }
}

#[test]
fn test_parse_factor() {}
