mod parser;
pub use parser::{
    FinConverter, InputParserFormat, OutputParserFormat, parse_input_and_serialize_via_trait,
};

// pub use ParseError;
pub use strum::ParseError;
