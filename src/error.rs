use std::fmt::{Display, Formatter};
use crate::{scanner, sexp};

#[derive(Debug)]
pub enum Error {
    ScanError(scanner::ScanError),
    SexpParseError(sexp::ParseError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}