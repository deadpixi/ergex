use std::error;
use std::fmt;

mod aho_corasick;
mod compiler;
mod intset;
mod queue;
mod transformers;

pub use compiler::{CompiledRegex, Database, DatabaseBuilder, Regex, Scratch};
pub use transformers::Transformer;

#[derive(Debug)]
pub enum Error {
    InvalidExpression(String),
    UnicodeWordBoundaries,
    TooManyRepetitions,
    SyntaxError(String),
    ProgramTooLarge,
}

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::InvalidExpression(ref msg) => write!(f, "invalid expression: {}", msg),
            Error::UnicodeWordBoundaries => {
                write!(f, "Unicode (non-)word boundaries are unsupported")
            }
            Error::TooManyRepetitions => write!(f, "maximum repetition count exceeded"),
            Error::SyntaxError(ref msg) => write!(f, "syntax error: {}", msg),
            Error::ProgramTooLarge => write!(f, "expression too large"),
        }
    }
}

impl From<regex_syntax::Error> for Error {
    fn from(error: regex_syntax::Error) -> Error {
        Error::SyntaxError(error.to_string())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Submatch {
    All,         // Report all submatches. Equivalent to AtMost(u32::MAX).
    AtMost(u32), // Report at most n submatches.
    Expression,  // Report the position of the expression as a whole. Equivalent to AtMost(0).
}

#[derive(Debug, Copy, Clone)]
pub enum MatchMode {
    First(Submatch), // Report only the first match.
    All(Submatch),   // Report all matches.
}

#[derive(Debug, Eq, PartialEq)]
pub enum Encoding {
    Byte, // Byte-at-a-time
    UTF8, // UTF8
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Capture {
    pub start: Option<usize>,
    pub end: Option<usize>,
}

impl Capture {
    fn new(start: Option<usize>, end: Option<usize>) -> Self {
        Self { start, end }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ContinueMatching {
    Yes,
    No,
}

pub trait MatchHandler {
    fn get_pulse_interval(&self) -> usize {
        1000
    }

    fn on_pulse(&mut self) -> ContinueMatching {
        ContinueMatching::Yes
    }

    fn on_reset(&mut self) {
        /* do nothing by default */
    }

    fn on_match(&mut self, id: usize, captures: &[Capture]) -> ContinueMatching;
}

#[cfg(test)]
mod tests;
