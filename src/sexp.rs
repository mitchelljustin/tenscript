use std::fmt::{Display, Formatter};
use crate::error::Error;
use crate::scanner;
use crate::scanner::{ScannedToken, Token};
use crate::scanner::Token::{Atom, Float, Ident, Integer, Paren, Percent, EOF};
use crate::sexp::ErrorKind::{ConsumeFailed, MatchExhausted};


#[derive(Debug, Clone)]
pub enum Literal {
    Atom(String),
    String(String),
    Integer(i64),
    Float(f64),
    Percent(f64),
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Atom(value) => write!(f, ":{value}"),
            Literal::String(value) => write!(f, "\"{value}\""),
            Literal::Percent(value) => write!(f, "{value}%"),
            Literal::Float(value) => write!(f, "{value}"),
            Literal::Integer(value) => write!(f, "{value}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Sexp {
    List { terms: Vec<Sexp> },
    Ident { name: String },
    Literal { value: Literal },
}

impl Display for Sexp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Sexp::List { terms } => {
                f.write_str("(")?;
                for (i, term) in terms.iter().enumerate() {
                    term.fmt(f)?;
                    if i < terms.len() - 1 {
                        f.write_str(" ")?;
                    }
                }
                f.write_str(")")?;
                Ok(())
            }
            Sexp::Ident { name } => write!(f, "{name}"),
            Sexp::Literal { value } => write!(f, "{value}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    kind: ErrorKind,
    token: ScannedToken,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    MatchExhausted,
    ConsumeFailed { expected: &'static str },
}

pub fn parse(source: &str) -> Result<Sexp, Error> {
    let tokens = scanner::scan(source)?;
    parse_tokens(tokens)
}

pub fn parse_tokens(tokens: Vec<ScannedToken>) -> Result<Sexp, Error> {
    Parser::new(tokens).parse().map_err(Error::SexpParseError)
}

struct Parser {
    tokens: Vec<ScannedToken>,
    index: usize,
}


impl Parser {
    pub fn new(tokens: Vec<ScannedToken>) -> Self {
        Self {
            tokens,
            index: 0,
        }
    }

    pub fn parse(mut self) -> Result<Sexp, ParseError> {
        self.sexp()
            .map_err(|kind| ParseError { kind, token: self.current_scanned().clone() })
    }

    fn current_scanned(&self) -> &ScannedToken {
        &self.tokens[self.index]
    }

    fn current(&self) -> &Token {
        &self.current_scanned().tok
    }



    fn increment(&mut self) {
        self.index += 1;
    }

    fn sexp(&mut self) -> Result<Sexp, ErrorKind> {
        let token = self.current().clone();
        self.increment();
        match token {
            Paren('(') =>
                self.list(),
            Ident(name) =>
                Ok(Sexp::Ident { name }),
            Float(value) =>
                Ok(Sexp::Literal { value: Literal::Float(value) }),
            Integer(value) =>
                Ok(Sexp::Literal { value: Literal::Integer(value) }),
            Percent(value) =>
                Ok(Sexp::Literal { value: Literal::Percent(value) }),
            Atom(value) =>
                Ok(Sexp::Literal { value: Literal::Atom(value) }),
            Token::String(value) =>
                Ok(Sexp::Literal { value: Literal::String(value) }),
            _ => Err(MatchExhausted),
        }
    }

    fn list(&mut self) -> Result<Sexp, ErrorKind> {
        let mut terms = Vec::new();
        while !matches!(self.current(), Paren(')') | EOF) {
            let term = self.sexp()?;
            terms.push(term);
        }
        let Paren(')') = self.current() else {
            return Err(ConsumeFailed { expected: "right paren" });
        };
        self.increment();
        Ok(Sexp::List { terms })
    }
}