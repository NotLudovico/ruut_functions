#![deny(missing_docs)]
//! Crate for creating math functions from string and perform symbolic derivation
use parser::{builder::build, to_rpn, ParsingError};
use simp::simp_node;

mod derivation;
mod display;
mod eval;
mod integration;
pub use crate::eval::{eval_vec_f1d, eval_vec_f2d, eval_vec_f3d};
mod macros;
mod ops;
mod param;
mod parser;
mod simp;
mod traits;

#[derive(Debug, PartialEq)]
/// Representation of a 1D function
pub struct F1D(Func);
#[derive(Debug, PartialEq)]
/// Representation of a 2D function
pub struct F2D(Func);
#[derive(Debug, PartialEq)]
/// Representation of a 3D function
pub struct F3D(Func);

impl F1D {
    /// Creates a new function from a str
    pub fn new(input: &str) -> Result<Self, ParsingError> {
        let mut func = build(to_rpn(input, &['x'])?);
        simp_node(&mut func);
        Ok(F1D(func))
    }
    /// Returns a string in latex format
    pub fn latex(&self) -> String {
        self.0.latex()
    }
}
impl F2D {
    /// Creates a new function from a str
    pub fn new(input: &str) -> Result<Self, ParsingError> {
        let mut func = build(to_rpn(input, &['x', 'y'])?);
        simp_node(&mut func);
        Ok(F2D(func))
    }

    /// Returns a string in latex format
    pub fn latex(&self) -> String {
        self.0.latex()
    }
}

impl F3D {
    /// Creates a new funcction from a str
    pub fn new(input: &str) -> Result<Self, ParsingError> {
        let mut func = build(to_rpn(input, &['x', 'y', 'z'])?);
        simp_node(&mut func);
        Ok(F3D(func))
    }
    /// Returns a string in latex format
    pub fn latex(&self) -> String {
        self.0.latex()
    }
}

impl FND {
    /// Creates a new function from a string
    pub fn new(input: &str, vars: &[char]) -> Result<Self, ParsingError> {
        let mut func = build(to_rpn(input, vars)?);
        simp_node(&mut func);
        Ok(FND {
            vars: vars.to_vec(),
            func,
        })
    }
}

#[derive(Debug, PartialEq)]
/// Representation of an n-dimensional function
pub struct FND {
    vars: Vec<char>,
    func: Func,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Func {
    Var(char),
    E,
    PI,
    Num(i32),
    Param(String, f64),
    Add(Vec<Self>),
    Mul(Vec<Self>),
    Pow(Box<Self>, Box<Self>),
    S(FType, Box<Self>),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum FType {
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

pub(crate) fn gcd(mut n: u32, mut m: u32) -> u32 {
    assert!(n != 0 && m != 0);
    if n == 0 {
        std::mem::swap(&mut n, &mut m);
    }
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
