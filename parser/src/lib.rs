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
//! use zlang_parser::{parser, ast};
//!
//! let zlang_source = "print('Hello world')";
//! let zlang_ast = parser::parse_expression(zlang_source).unwrap();
//!
//! ```

#[macro_use]

extern crate log;

pub mod token;
pub mod location;
pub mod error;
pub mod lexer;
