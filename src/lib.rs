use parser::{builder::build, to_rpn, ParsingError};
use simp::simp_node;

pub mod ops;
pub mod parser;
mod simp;
mod traits;

#[derive(Debug, PartialEq)]
pub(crate) struct Ctx<'a>(Vec<&'a str>);
impl<'a> Ctx<'a> {
    pub fn new(symbols: &'a [&str]) -> Self {
        Ctx(symbols.to_vec())
    }
}

#[derive(Debug, PartialEq)]
pub struct F1D<'a> {
    func: Func,
    ctx: Ctx<'a>,
}

impl<'a> F1D<'a> {
    pub fn new(input: &str) -> Result<Self, ParsingError> {
        let mut func = build(to_rpn(input, &['x'])?);
        simp_node(&mut func);
        Ok(F1D {
            func,
            ctx: Ctx::new(&[]),
        })
    }
}
pub struct F2D<'a> {
    func: Func,
    ctx: Ctx<'a>,
}
pub struct F3D<'a> {
    func: Func,
    ctx: Ctx<'a>,
}

pub struct FND<'a> {
    vars: Vec<&'a str>,
    func: Func,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Func {
    Var(char),
    E,
    PI,
    Num(i32),
    Param(String),
    Add(Vec<Self>),
    Mul(Vec<Self>),
    Pow(Box<Self>, Box<Self>),
    S(FType, Box<Self>),
}

#[derive(Debug, PartialEq, Clone)]
enum FType {
    Sin,
    Cos,
    Tan,
    Cot,
    Sec,
    Csc,
    ASin,
    ACos,
    ATan,
    Sinh,
    Cosh,
    Tanh,
    Coth,
    Sech,
    Csch,
    ASinh,
    ACosh,
    ATanh,
    Abs,
    Ln,
}
