use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use crate::error::Error;
use crate::interpreter::ErrorKind::{BadCall, IllegalCall, Mismatch};
use crate::sexp;
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

impl Default for VulcanizeType {
    fn default() -> Self {
        VulcanizeType::Bowtie
    }
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

impl Default for SeedType {
    fn default() -> Self {
        SeedType::Left
    }
}

#[derive(Debug, Clone)]
pub enum Twist {
    Continue,
    Switch,
}

#[derive(Debug, Clone)]
pub enum GrowthInstruction {
    Noop,
    Grow {
        face: Face,
        repeat: Option<usize>,
        instructions: Vec<GrowthInstruction>,
    },
    Branch { faces: HashMap<Face, GrowthInstruction> },
    Twist { twists: Vec<Twist> },
    Mark { name: String },
    Join { name: String },
}

impl Default for GrowthInstruction {
    fn default() -> Self {
        Self::Noop
    }
}

#[derive(Debug, Clone, Default)]
pub struct BuildPhase {
    seed: Option<SeedType>,
    scale: Option<f64>,
    vulcanize: Option<VulcanizeType>,
    growth: Option<GrowthInstruction>,
}


type Features = HashMap<String, Value>;

#[derive(Debug, Clone, Default)]
pub struct Fabric {
    name: Option<String>,
    scale: Option<f64>,
    features: Features,
    build_phase: BuildPhase,
}

#[derive(Debug, Clone)]
pub struct InterpretError {
    kind: ErrorKind,
}

impl Display for InterpretError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Mismatch { rule: &'static str, sexp: Sexp, expected: &'static str },
    BadCall { context: &'static str, expected: &'static str, sexp: Sexp },
    TypeError { expected: &'static str, sexp: Sexp },
    AlreadyDefined { property: &'static str, sexp: Sexp },
    IllegalCall { context: &'static str, sexp: Sexp },
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

pub fn interpret(source: &str) -> Result<Fabric, Error> {
    interpret_sexp(&sexp::parse(source)?)
}

pub fn interpret_sexp(sexp: &Sexp) -> Result<Fabric, Error> {
    builder::interpret(sexp)
        .map_err(Error::InterpretError)
}

mod builder {
    use crate::interpreter::{ErrorKind, Fabric, GrowthInstruction, InterpretError, SeedType, Value, VulcanizeType};
    use crate::interpreter::ErrorKind::{AlreadyDefined, BadCall, IllegalCall, Mismatch};
    use crate::sexp::Sexp;

    macro_rules! expect_enum {
        ($value:expr, { $($name:literal => $enum_val:expr,)+ }) => {
            {
                let expected = stringify!($($name)|+);
                let $crate::sexp::Sexp::Atom(ref name) = $value else {
                    return Err($crate::interpreter::ErrorKind::TypeError { expected, sexp: $value.clone() })
                };
                match name.as_str() {
                    $(
                        $name => $enum_val,
                    )+
                    _ => return Err($crate::interpreter::ErrorKind::TypeError { expected, sexp: $value.clone() })
                }
            }
        }
    }

    struct Call<'a> {
        head: &'a str,
        tail: &'a [Sexp],
    }

    pub fn interpret(sexp: &Sexp) -> Result<Fabric, InterpretError> {
        fabric(sexp)
            .map_err(|kind| InterpretError { kind })
    }

    fn expect_call<'a>(rule: &'static str, sexp: &'a Sexp) -> Result<Call<'a>, ErrorKind> {
        let Sexp::List(ref terms) = sexp else {
            return Err(Mismatch { rule, expected: "( .. )", sexp: sexp.clone() });
        };
        let [ref head, ref tail @ ..] = terms[..] else {
            return Err(Mismatch { rule, expected: "(<head> ..)", sexp: sexp.clone() });
        };
        let Sexp::Ident(ref head) = head else {
            return Err(Mismatch { rule, expected: "(<head:ident> ..)", sexp: sexp.clone() });
        };
        Ok(Call {
            head,
            tail,
        })
    }

    fn fabric(sexp: &Sexp) -> Result<Fabric, ErrorKind> {
        let Call { head: "fabric", tail } = expect_call("fabric", sexp)? else {
            return Err(Mismatch { rule: "fabric", expected: "(fabric ..)", sexp: sexp.clone() });
        };

        let mut fabric = Fabric::default();
        for sexp in tail {
            let Call { head, tail } = expect_call("fabric", sexp)?;
            match head {
                "scale" => {
                    if let Some(_) = fabric.scale {
                        return Err(AlreadyDefined { property: "scale", sexp: sexp.clone() });
                    };
                    let &[Sexp::Percent(scale)] = tail else {
                        return Err(BadCall { context: "fabric def", expected: "(scale <percent>)", sexp: sexp.clone() });
                    };
                    fabric.scale = Some(scale / 100.0);
                }
                "name" => {
                    if let Some(_) = fabric.name {
                        return Err(AlreadyDefined { property: "name", sexp: sexp.clone() });
                    };
                    let &[Sexp::String(ref name)] = tail else {
                        return Err(BadCall { context: "fabric def", expected: "(name <string>)", sexp: sexp.clone() });
                    };
                    fabric.name = Some(name.clone());
                }
                "features" => {
                    features(&mut fabric, tail)?;
                }
                "build" => {
                    build(&mut fabric, tail)?;
                }
                "shape" => {}
                "pretense" => {}
                _ => return Err(IllegalCall { context: "fabric def", sexp: sexp.clone() })
            }
        }
        Ok(fabric)
    }

    fn build(Fabric { build_phase, .. }: &mut Fabric, sexps: &[Sexp]) -> Result<(), ErrorKind> {
        for sexp in sexps {
            let Call { head, tail } = expect_call("build", sexp)?;
            match head {
                "seed" => {
                    if let Some(_) = build_phase.seed {
                        return Err(AlreadyDefined { property: "seed", sexp: sexp.clone() });
                    };
                    let &[ref value] = tail else {
                        return Err(BadCall { context: "build phase", expected: "(seed <value>)", sexp: sexp.clone() });
                    };
                    let seed_type = expect_enum!(value, {
                        "left" => SeedType::Left,
                        "left-right" => SeedType::LeftRight,
                    });
                    build_phase.seed = Some(seed_type);
                }
                "vulcanize" => {
                    if let Some(_) = build_phase.vulcanize {
                        return Err(AlreadyDefined { property: "vulcanize", sexp: sexp.clone() });
                    };

                    let &[ref value] = tail else {
                        return Err(BadCall { context: "build phase", expected: "(vulcanize <value>)", sexp: sexp.clone() });
                    };
                    let vulcanize_type = expect_enum!(value, {
                        "bowtie" => VulcanizeType::Bowtie,
                        "snelson" => VulcanizeType::Snelson,
                    });
                    build_phase.vulcanize = Some(vulcanize_type);
                }
                "scale" => {
                    if let Some(_) = build_phase.scale {
                        return Err(AlreadyDefined { property: "scale", sexp: sexp.clone() });
                    };
                    let &[Sexp::Percent(value)] = tail else {
                        return Err(BadCall { context: "build phase", expected: "(scale <percent>)", sexp: sexp.clone() });
                    };
                    build_phase.scale = Some(value);
                }
                "branch" | "grow" => {
                    if let Some(_) = build_phase.growth {
                        return Err(AlreadyDefined { property: "growth", sexp: sexp.clone() });
                    };
                    build_phase.growth = Some(growth_instruction(sexp)?);
                }
                _ => return Err(IllegalCall { context: "build phase", sexp: sexp.clone() })
            }
        }
        Ok(())
    }

    fn growth_instruction(sexp: &Sexp) -> Result<GrowthInstruction, ErrorKind> {
        todo!()
    }

    fn features(Fabric { features, .. }: &mut Fabric, sexps: &[Sexp]) -> Result<(), ErrorKind> {
        for sexp in sexps {
            let Call { head: key, tail: &[ref val] } = expect_call("features", sexp)? else {
                return Err(BadCall { context: "features", expected: "(<feature-name> <value>)", sexp: sexp.clone() });
            };
            features.insert(key.to_string(), literal(val)?);
        }
        Ok(())
    }

    fn literal(sexp: &Sexp) -> Result<Value, ErrorKind> {
        let value = match sexp {
            Sexp::Atom(value) => Value::Atom(value.clone()),
            Sexp::String(value) => Value::String(value.clone()),
            Sexp::Integer(value) => Value::Integer(value.clone()),
            Sexp::Float(value) => Value::Float(value.clone()),
            Sexp::Percent(value) => Value::Percent(value.clone()),
            _ => return Err(Mismatch { rule: "literal", expected: "<atom|string|integer|float|percent>", sexp: sexp.clone() }),
        };
        Ok(value)
    }
}

