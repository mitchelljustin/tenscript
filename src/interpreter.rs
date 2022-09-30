use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use crate::error::Error;
use crate::interpreter::ErrorKind::Mismatch;
use crate::sexp::Sexp;

#[derive(Debug, Clone)]
pub enum Value {
    Atom(String),
    String(String),
    Integer(i64),
    Float(f64),
    Percent(f64),
}

#[derive(Debug, Clone)]
pub enum Face {
    A,
    B,
    C,
    D,
    a,
    b,
    c,
    d,
}

#[derive(Debug, Clone)]
pub enum VulcanizeType {
    Bowtie,
    Snelson,
}

#[derive(Debug, Clone)]
pub enum SurfaceCharacter {
    Frozen,
    Bouncy,
}

#[derive(Debug, Clone)]
pub enum SeedType {
    Left,
    LeftRight,
}

#[derive(Debug, Clone)]
pub enum Twist {
    Continue,
    Switch,
}

#[derive(Debug, Clone)]
pub enum BuildInstruction {
    Seed { ty: SeedType },
    SetScale { scale: f64 },
    Grow {
        face: Face,
        repeat: Option<usize>,
        instructions: Vec<BuildInstruction>,
    },
    Twist { twists: Vec<Twist> },
    Branch { faces: HashMap<Face, BuildInstruction> },
    Mark { name: String },
    Vulcanize { ty: VulcanizeType },
    Join { name: String },
}

#[derive(Debug, Clone, Default)]
pub struct BuildPhase {
    instructions: Vec<BuildInstruction>,
}

#[derive(Debug, Clone, Default)]
pub struct Fabric {
    name: Option<String>,
    scale: Option<f64>,
    features: HashMap<String, Value>,
    build_phase: BuildPhase,
}

pub struct Interpreter {}

#[derive(Debug)]
pub struct InterpretError {
    kind: ErrorKind,
}

impl Display for InterpretError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Mismatch { actual: Box<dyn Debug>, expected: &'static str },
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

pub fn interpret_sexp(sexp: Sexp) -> Result<Fabric, Error> {
    Interpreter::new()
        .interpret(sexp)
        .map_err(Error::InterpretError)
}

impl Interpreter {
    fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, sexp: Sexp) -> Result<Fabric, InterpretError> {
        self.fabric(sexp).map_err(|kind| InterpretError { kind })
    }

    fn fabric(&mut self, sexp: Sexp) -> Result<Fabric, ErrorKind> {
        let Sexp::List(terms) = sexp else {
            return Err(Mismatch { expected: "sexp list", actual: Box::new(sexp) });
        };
        let Some(Sexp::String(head)) = terms.first() else {
            return Err(Mismatch { expected: "fabric sexp head", actual: Box::new(terms.first().clone()) });
        };
        if head != "fabric" {
            return Err(Mismatch { expected: "'fabric' sexp head", actual: Box::new(head) });
        }
        let mut fabric = Fabric::default();

        Ok(fabric)
    }
}

