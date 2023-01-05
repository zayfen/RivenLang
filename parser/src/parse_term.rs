use crate::ast::Term;
use crate::parse_primary::parse_primary;
use crate::parser::Parser;

pub fn parse_term(parser: &mut Parser) -> Result<Term, String> {
  parse_primary(parser);

  Err("N/A".to_owned())
}
