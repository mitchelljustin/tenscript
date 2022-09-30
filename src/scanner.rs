use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;
use crate::scanner::ErrorKind::{FloatParseFailed, IllegalChar, IntParseFailed};
use crate::scanner::Token::{Atom, EOF, Ident, Integer, Number, Percent, Sym};

#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    Atom(String),
    StringLit(String),
    Sym(&'static str),
    Integer(i64),
    Number(f64),
    Percent(f64),
    EOF,
}

#[derive(Debug, Clone, Default)]
pub struct Location {
    line: usize,
    col: usize,
}

#[derive(Debug, Clone)]
pub struct ScannedToken {
    pub(crate) tok: Token,
    loc: Location,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Location { line, col } = self;
        write!(f, "{line}:{col}")
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    loc: Location,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Error { kind, loc } = self;
        write!(f, "{kind:?} at {loc}")
    }
}

impl std::error::Error for Error {}


#[derive(Debug)]
pub enum ErrorKind {
    IllegalChar { ch: char },
    IntParseFailed { err: ParseIntError },
    FloatParseFailed { err: ParseFloatError },
}

pub fn scan(source: &str) -> Result<Vec<ScannedToken>, Error> {
    Scanner::new(source).scan()
}

struct Scanner {
    chars: Vec<char>,
    tokens: Vec<ScannedToken>,
    index: usize,
    start: usize,
    loc: Location,
}

const SYMS: &[&'static str] = &[
    "(",
    ")",
];

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            tokens: Default::default(),
            start: 0,
            index: 0,
            loc: Default::default(),
        }
    }

    pub fn scan(mut self) -> Result<Vec<ScannedToken>, Error> {
        while !self.at_end() {
            self.start = self.index;
            self.scan_token()
                .map_err(|kind| Error { kind, loc: self.loc.clone() })?;
        }
        self.add(EOF);
        Ok(self.tokens)
    }


    fn scan_token(&mut self) -> Result<(), ErrorKind> {
        match self.current() {
            '0'..='9' | '-' => self.number()?,
            'a'..='z' | 'A'..='Z' | '-' => self.ident(),
            ':' => self.atom(),
            '\n' => {
                self.loc.line += 1;
                self.loc.col = 0;
                self.increment();
            }
            ' ' | '\t' => {
                self.increment()
            }
            ch => {
                let ch_str = ch.to_string();
                let Some(&sym) = SYMS.iter().find(|&&s| s == ch_str) else {
                    return Err(IllegalChar { ch });
                };
                self.increment();
                self.add(Sym(sym));
            }
        }
        Ok(())
    }


    fn at_end(&self) -> bool {
        self.index >= self.chars.len()
    }

    fn current(&self) -> char {
        self.chars[self.index]
    }

    fn increment(&mut self) {
        self.index += 1;
        self.loc.col += 1;
    }


    fn add(&mut self, tok: Token) {
        self.tokens.push(ScannedToken {
            tok,
            loc: self.loc.clone(),
        })
    }

    fn lexeme(&self) -> String {
        self.chars[self.start..self.index].iter().collect()
    }

    fn consume_ident_chars(&mut self) {
        let ('a'..='z' | 'A'..='Z' | '-') = self.current() else {
            return;
        };
        self.increment();
        while let 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' = self.current() {
            self.increment();
        }
    }

    fn number(&mut self) -> Result<(), ErrorKind> {
        let negative = if let '-' = self.current() {
            self.increment();
            true
        } else { false };
        while let '0'..='9' = self.current() {
            self.increment();
        }
        if let '.' = self.current() {
            self.increment();
            while let '0'..='9' = self.current() {
                self.increment();
                let mut value = f64::from_str(&self.lexeme())
                    .map_err(|err| FloatParseFailed { err })?;
                if negative {
                    value = -value;
                }
                match self.current() {
                    '%' => {
                        self.increment();
                        self.add(Percent(value));
                    }
                    _ => self.add(Number(value)),
                };
            }
        } else {
            let mut value = i64::from_str(&self.lexeme())
                .map_err(|err| IntParseFailed { err })?;;
            if negative {
                value = -value;
            }
            match self.current() {
                '%' => {
                    self.increment();
                    self.add(Percent(value as f64));
                }
                _ => self.add(Integer(value)),
            };
        }

        Ok(())
    }

    fn atom(&mut self) {
        self.increment(); // consume ':'
        self.consume_ident_chars();
        let mut name = self.lexeme();
        name.remove(0); // remove prefix ':'
        self.add(Atom(name));
    }

    fn ident(&mut self) {
        self.consume_ident_chars();
        let name = self.lexeme();
        self.add(Ident(name));
    }
}