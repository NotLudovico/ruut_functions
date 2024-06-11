use crate::{FType, Func, F1D, F2D, F3D};

impl<'a> F1D<'a> {
    pub fn derive(&self, order: usize) -> Self {
        F1D {
            func: self.func.derive_nth('x', order),
            ctx: self.ctx.clone(),
        }
    }
}
impl<'a> F2D<'a> {
    pub fn derive(&self, var: char, order: usize) -> Self {
        F2D {
            func: self.func.derive_nth(var, order),
            ctx: self.ctx.clone(),
        }
    }
    pub fn gradient(&self) -> Vec<Self> {
        vec![self.derive('x', 1), self.derive('y', 1)]
    }
    pub fn hessian(&self) -> Vec<Vec<Self>> {
        vec![
            vec![self.derive('x', 2), self.derive('x', 1).derive('y', 1)],
            vec![self.derive('y', 1).derive('x', 1), self.derive('y', 2)],
        ]
    }
}

impl<'a> F3D<'a> {
    pub fn derive(&self, var: char, order: usize) -> Self {
        F3D {
            func: self.func.derive_nth(var, order),
            ctx: self.ctx.clone(),
        }
    }
    pub fn gradient(&self) -> Vec<Self> {
        vec![
            self.derive('x', 1),
            self.derive('y', 1),
            self.derive('z', 1),
        ]
    }
    pub fn hessian(&self) -> Vec<Vec<Self>> {
        vec![
            vec![
                self.derive('x', 2),
                self.derive('x', 1).derive('y', 1),
                self.derive('x', 1).derive('z', 1),
            ],
            vec![
                self.derive('y', 1).derive('x', 1),
                self.derive('y', 2),
                self.derive('y', 1).derive('z', 1),
            ],
            vec![
                self.derive('z', 1).derive('x', 1),
                self.derive('z', 1).derive('y', 1),
                self.derive('z', 2),
            ],
        ]
    }
}
impl Func {
    fn derive_nth(&self, var: char, order: usize) -> Self {
        let mut result = self.clone();
        for _ in 1..=order {
            result = result.derive(var);
        }
        result
    }
    fn derive(&self, var: char) -> Self {
        match self {
            Self::Var(char) => {
                if *char == var {
                    Self::Num(1)
                } else {
                    Self::Num(0)
                }
            }
            Self::Num(_) | Self::Param(_) => Self::Num(0),
            Self::E | Self::PI => Self::Num(0),
            Self::Add(add) => add.iter().map(|term| term.derive(var)).sum::<Self>(),
            Self::Mul(mul) => {
                let mut deriv_multipliers = Func::Num(0);
                let mut multipliers = Func::Num(1);
                for (i, term) in mul.iter().enumerate() {
                    multipliers = multipliers * term.clone();
                    let mut prods = term.derive(var);
                    for (j, other) in mul.iter().enumerate() {
                        if i != j {
                            prods = prods * other.clone()
                        }
                    }
                    deriv_multipliers += prods;
                }

                deriv_multipliers
            }
            Self::Pow(base, exp) => {
                if let Func::E = **base {
                    return exp.derive(var) * self.clone();
                }
                if let Func::Num(exp_val) = **exp {
                    return exp_val * base.derive(var) * base.clone().powi(exp_val - 1);
                }
                (Func::E.pow(*base.clone() * Self::S(FType::Ln, exp.clone()))).derive(var)
            }
            Self::S(kind, argument) => {
                let argument = Box::new(*argument.clone());
                let arg = argument.derive(var);

                match kind {
                    FType::Ln => arg / *argument,
                    FType::Sin => arg * Func::S(FType::Cos, argument),
                    FType::Cos => -1 * arg * Func::S(FType::Sin, argument),
                    FType::Tan => arg * Func::S(FType::Sec, argument).powi(2),
                    FType::Cot => -1 * arg * (Func::S(FType::Csc, argument)).powi(2),
                    FType::Sec => {
                        arg * Func::S(FType::Sec, argument.clone()) * Func::S(FType::Tan, argument)
                    }
                    FType::Csc => {
                        -1 * arg
                            * Func::S(FType::Cot, argument.clone())
                            * Func::S(FType::Csc, argument)
                    }
                    FType::ASin => arg / (1 - argument.powi(2)).pow(Func::Num(1) / Func::Num(2)),
                    FType::ACos => {
                        -1 * arg / (1 - argument.powi(2)).pow(Func::Num(1) / Func::Num(2))
                    }
                    FType::ATan => arg / (1 + argument.powi(2)),
                    FType::Sinh => arg * Func::S(FType::Cosh, argument),
                    FType::Cosh => arg * Func::S(FType::Sinh, argument),
                    FType::Tanh => arg * Func::S(FType::Sech, argument).powi(2),
                    FType::Coth => -1 * arg * Func::S(FType::Csch, argument).powi(2),
                    FType::Sech => {
                        -1 * arg
                            * Func::S(FType::Sech, argument.clone())
                            * Func::S(FType::Tanh, argument)
                    }
                    FType::Csch => {
                        -1 * arg
                            * Func::S(FType::Csch, argument.clone())
                            * Func::S(FType::Coth, argument)
                    }
                    FType::ASinh => arg / (1 + argument.powi(2)).pow(Func::Num(1) / Func::Num(2)),
                    FType::ACosh => arg / (argument.powi(2) - 1).pow(Func::Num(1) / Func::Num(2)),
                    FType::ATanh => arg / (1 - argument.powi(2)),
                    FType::Abs => arg * *argument.clone() / Func::S(FType::Abs, argument),
                }
            }
        }
    }
}

#[test]
fn test_derive() {
    assert_eq!(
        F1D::new("x+ln(x)+x^2+sin(2x)").unwrap().derive(1),
        F1D::new("1+1/x+2x+2cos(2x)").unwrap()
    )
}
