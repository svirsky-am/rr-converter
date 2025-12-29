#![warn(missing_docs)]

// This module provides the main API for parsing and converting financial data
// between supported input and output formats via the `FinConverter` and
// `parse_input_and_serialize_via_trait` function.
mod parser;
// use key types and functions for public use.
pub use parser::{
    FinConverter, InputParserFormat, OutputParserFormat, parse_input_and_serialize_via_trait,
};

// pub use ParseError;
pub use strum::ParseError;
