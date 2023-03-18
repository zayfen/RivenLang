//! This crate can be used to parse zlang sourcecode into a so
//! called AST (abstract syntax tree).
//!
//! The stages involved in this process are lexical analysis and
//! parsing. The lexical analysis splits the sourcecode into
//! tokens, and the parsing transforms those tokens into an AST.
//!
//! For example, one could do this:
//!
//! ```
//! use parser::lexer::make_tokenizer;
//! use std::iter::FromIterator;
//!
//! let zlang_source = "print('Hello world')".to_owned();
//! let lxr = make_tokenizer(&zlang_source);
//! let zlang_tokens = Vec::from_iter(lxr.map(|x| x.unwrap().1));
//!
//! ```

#[macro_use]
extern crate log;

pub mod ast;
pub mod error;
pub mod lexer;
pub mod location;
pub mod parser;
pub mod token;

pub mod parse_primary;
pub mod parse_factor;
pub mod parse_term;
pub mod parse_arithmetic_expr;
pub mod parse_call_expr;
pub mod parse_expression;
pub mod parse_expression_list;
pub mod parse_assign_statement;
pub mod parse_return_statement;
pub mod parse_if_stmt;
pub mod parse_function_stmt;
pub mod parse_stmt;
pub mod parse_stmt_list;
pub mod parse_program;

pub mod parse_logic_expr;

pub mod parse_component_factor;

pub mod parse_component_term;

pub mod parse_component_arithmetic_expr;

// pub mod parse_literal;
// pub mod parse_primary;
// pub mod parse_term;

// pub mod parse_expression;
