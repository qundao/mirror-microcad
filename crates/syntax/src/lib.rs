use std::ops::Range;

pub type Span = Range<usize>;

pub mod tokens;
pub mod ast;
pub mod parser;
