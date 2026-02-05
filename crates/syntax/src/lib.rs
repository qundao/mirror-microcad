use std::ops::Range;

pub type Span = Range<usize>;

pub mod ast;
pub mod parser;
pub mod tokens;
