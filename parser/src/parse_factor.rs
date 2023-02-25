use crate::ast::{Factor, FactorValue, Identifier};
use crate::parse_primary::{match_primary, parse_primary};
use crate::parser::Parser;
use crate::token::Token;

pub(crate) fn match_factor(token: Token) -> bool {
  match_primary(token.clone()) || matches!(token, Token::Id { name: _ })
}

pub fn parse_factor(parser: &mut Parser) -> Factor {
  let token = parser.get_token();
  match token {
    Token::Id { name } => {
      parser.advance_token();
      return Factor::from(FactorValue::Identifier(Identifier::from(name.as_str())));
    }
    _ => Factor::from(FactorValue::Primary(parse_primary(parser))),
  }
}
