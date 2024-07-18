use crate::{FType, Func, F1D, F2D, F3D, FND};

impl F1D {
    /// Computes the nth-derivative
    /// ```
    /// use ruut_functions::{f1d, F1D};
    ///
    /// let f = f1d!("x^log(x)");
    /// let df = f.derive(1);
    /// assert_eq!(df, f1d!("2ln(x)x^ln(x)/x"));
    /// assert_eq!(df.derive(1), f.derive(2));
    /// ```
    pub fn derive(&self, order: usize) -> Self {
        F1D(self.0.derive_nth('x', order))
    }
}
impl F2D {
    /// Computes the nth-derivative
    /// ```
    /// use ruut_functions::{f2d,F2D};
    /// assert_eq!(f2d!("x+y^2").derive('y', 2), f2d!("2"));
    /// ```
    pub fn derive(&self, var: char, order: usize) -> Self {
        F2D(self.0.derive_nth(var, order))
    }
    /// Computes the gradient
    /// ```
    /// use ruut_functions::{f2d, F2D};
    /// assert_eq!(f2d!("x+y^2").gradient(), vec![f2d!("1"), f2d!("2y")]);
    /// ```
    pub fn gradient(&self) -> Vec<Self> {
        vec![self.derive('x', 1), self.derive('y', 1)]
    }

    /// Computes the hessian matrix
    /// ```
    /// use ruut_functions::{f2d, F2D};
    /// let f = f2d!("x^3+y^2");
    /// let hessian = f.hessian();
    /// assert_eq!(hessian, vec![vec![f2d!("6x"), f2d!("0")],
    ///                          vec![f2d!("0"), f2d!("2")]]);
    /// ```
    pub fn hessian(&self) -> Vec<Vec<Self>> {
        vec![
            vec![self.derive('x', 2), self.derive('x', 1).derive('y', 1)],
            vec![self.derive('y', 1).derive('x', 1), self.derive('y', 2)],
        ]
    }
}

impl F3D {
    /// Computes the nth-derivative
    /// ```
    /// use ruut_functions::{f3d, F3D};
    /// assert_eq!(f3d!("x+zy^2").derive('y', 2), f3d!("2z"));
    /// ```
    pub fn derive(&self, var: char, order: usize) -> Self {
        F3D(self.0.derive_nth(var, order))
    }
    /// Computes the gradient
    /// ```
    /// use ruut_functions::{f3d, F3D};
    /// assert_eq!(f3d!("x+zy^2").gradient(), vec![f3d!("1"), f3d!("2yz"), f3d!("y^2")]);
    /// ```
    pub fn gradient(&self) -> Vec<Self> {
        vec![
            self.derive('x', 1),
            self.derive('y', 1),
            self.derive('z', 1),
        ]
    }
    /// Computes the hessian
    /// ```
    /// use ruut_functions::{f3d, F3D};
    /// assert_eq!(f3d!("x^3+zy^2").hessian(), vec![vec![f3d!("6x"), f3d!("0"), f3d!("0")],
    ///                                             vec![f3d!("0"), f3d!("2z"), f3d!("2y")],
    ///                                             vec![f3d!("0"), f3d!("2y"), f3d!("0")]]);
    /// ```
    pub fn hessian(&self) -> Vec<Vec<Self>> {
        let dx = self.derive('x', 1);
        let dy = self.derive('y', 1);
        let dz = self.derive('z', 1);
        vec![
            vec![dx.derive('x', 1), dx.derive('y', 1), dx.derive('z', 1)],
            vec![dy.derive('x', 1), dy.derive('y', 1), dy.derive('z', 1)],
            vec![dz.derive('x', 1), dz.derive('y', 1), dz.derive('z', 1)],
        ]
    }
}

impl FND {
    /// Computes the nth-derivative wrt a variable
    /// ```
    /// use ruut_functions::{fnd,FND};
    /// let vars = ['f', 'z'];
    /// assert_eq!(fnd!("f^2+z", &vars).derive('f', 1), fnd!("2f", &vars));
    /// ```
    pub fn derive(&self, var: char, order: usize) -> Self {
        FND {
            vars: self.vars.clone(),
            func: self.func.derive_nth(var, order),
        }
    }

    /// Computes the gradient
    /// ```
    /// use ruut_functions::{fnd,FND};
    /// let vars = ['f', 'z'];
    /// assert_eq!(fnd!("f^2+z", &vars).gradient(), vec![fnd!("2f", &vars), fnd!("1", &vars)]);
    /// ```
    pub fn gradient(&self) -> Vec<Self> {
        let mut result = Vec::with_capacity(self.vars.len());
        for var in &self.vars {
            result.push(self.derive(*var, 1));
        }
        result
    }
    /// Computes the hessian
    /// ```
    /// use ruut_functions::{fnd,FND};
    /// let vars = ['f', 'z'];
    /// assert_eq!(fnd!("f^3+zf", &vars).hessian(), vec![vec![fnd!("6f", &vars), fnd!("1", &vars)],
    ///                                                  vec![fnd!("1", &vars), fnd!("0", &vars)]]);
    /// ```
    pub fn hessian(&self) -> Vec<Vec<Self>> {
        let mut result = Vec::new();

        // first derivative
        let mut first_deriv = Vec::with_capacity(self.vars.len());
        for var in &self.vars {
            first_deriv.push(self.derive(*var, 1));
        }

        for el in first_deriv {
            let mut gradient = Vec::with_capacity(self.vars.len());

            for var in &self.vars {
                gradient.push(el.derive(*var, 1))
            }
            result.push(gradient);
        }

        result
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
        let res = match self {
            Self::Var(char) => {
                if *char == var {
                    Self::Num(1)
                } else {
                    Self::Num(0)
                }
            }
            Self::Num(_) | Self::Param(..) => Self::Num(0),
            Self::E | Self::PI => Self::Num(0),
            Self::Add(add) => add.iter().map(|term| term.derive(var)).sum::<Self>(),
            Self::Mul(mul) => {
                let mut result = Func::Num(0);
                for (i, term) in mul.iter().enumerate() {
                    let mut multipliers = term.derive(var);
                    for (j, other) in mul.iter().enumerate() {
                        if i != j {
                            multipliers *= other.clone()
                        }
                    }
                    result += multipliers;
                }

                result
            }
            Self::Pow(base, exp) => {
                if let Func::E = **base {
                    return exp.derive(var) * self.clone();
                }
                if let Func::Num(exp_val) = **exp {
                    return exp_val * base.derive(var) * base.clone().powi(exp_val - 1);
                }
                (Func::E.pow(*exp.clone() * Self::S(FType::Ln, base.clone()))).derive(var)
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
        };
        res
    }
}

#[test]
fn test_derive() {
    use crate::{f1d, f2d, f3d, fnd};

    assert_eq!(
        f1d!("x+ln(x)+x^2+sin(2x)").derive(1),
        f1d!("1+1/x+2cos(2x)+2x")
    );

    assert_eq!(f1d!("3x+7+e").derive(1), f1d!("3"));
    assert_eq!(f1d!("xsin(x)").derive(1), f1d!("sin(x)+xcos(x)"));
    assert_eq!(f1d!("tan(x^2)").derive(1), f1d!("2xsec(x^2)^2"));
    assert_eq!(f1d!("x^x").derive(1), f1d!("(ln(x)+1)e^(xln(x))"));
    assert_eq!(
        f3d!("xyz^2").gradient(),
        vec![f3d!("yz^2"), f3d!("xz^2"), f3d!("2xyz")]
    );
    // assert_eq!(f1d!("x/(x+1)").derive(1), f1d!("1/(x+1)^2"));
    assert_eq!(f1d!("1/(3e*x^2)").derive(1), f1d!("-2/(3e*x^3)"));
    assert_eq!(f1d!("cos(x)").derive(1), f1d!("-sin(x)"));
    assert_eq!(f1d!("sin(x)").derive(1), f1d!("cos(x)"));
    assert_eq!(f1d!("cot(x)").derive(1), f1d!("-csc(x)^2"));
    assert_eq!(f1d!("sec(x)").derive(1), f1d!("sec(x)tan(x)"));
    assert_eq!(f1d!("csc(x)").derive(1), f1d!("-csc(x)cot(x)"));
    assert_eq!(f1d!("asin(x)").derive(1), f1d!("1/(1-x^2)^(1/2)"));
    assert_eq!(f1d!("acos(x)").derive(1), f1d!("-1/(1-x^2)^(1/2)"));
    assert_eq!(f1d!("atan(x)").derive(1), f1d!("1/(1+x^2)"));
    assert_eq!(f1d!("sinh(x)").derive(1), f1d!("cosh(x)"));
    assert_eq!(f1d!("cosh(x)").derive(1), f1d!("sinh(x)"));
    assert_eq!(f1d!("tanh(x)").derive(1), f1d!("sech(x)^2"));
    assert_eq!(f1d!("coth(x)").derive(1), f1d!("-csch(x)^2"));
    assert_eq!(f1d!("sech(x)").derive(1), f1d!("-tanh(x)sech(x)"));
    assert_eq!(f1d!("csch(x)").derive(1), f1d!("-csch(x)coth(x)"));
    assert_eq!(f1d!("asinh(x)").derive(1), f1d!("1/(1+x^2)^(1/2)"));
    assert_eq!(f1d!("acosh(x)").derive(1), f1d!("1/(x^2-1)^(1/2)"));
    assert_eq!(f1d!("atanh(x)").derive(1), f1d!("1/(1-x^2)"));
    assert_eq!(f1d!("abs(x)").derive(1), f1d!("x/abs(x)"));

    // F2D
    assert_eq!(f2d!("xy+y^2").gradient(), vec![f2d!("y"), f2d!("x+2y")]);
    assert_eq!(
        f2d!("xy+y^2").hessian(),
        vec![vec![f2d!("0"), f2d!("1")], vec![f2d!("1"), f2d!("2")]]
    );

    // F3D
    assert_eq!(
        f3d!("xy+y^2+1/z").gradient(),
        vec![f3d!("y"), f3d!("x+2y"), f3d!("-1/z^2")]
    );
    assert_eq!(
        f3d!("xy+y^2+1/z").hessian(),
        vec![
            vec![f3d!("0"), f3d!("1"), f3d!("0")],
            vec![f3d!("1"), f3d!("2"), f3d!("0")],
            vec![f3d!("0"), f3d!("0"), f3d!("2/z^3")]
        ]
    );

    // FND
    let v = ['w', 'f'];
    assert_eq!(
        fnd!("w+f^2", &v).gradient(),
        vec![fnd!("1", &v), fnd!("2f", &v)]
    );

    assert_eq!(
        fnd!("w+f^2", &v).hessian(),
        vec![
            vec![fnd!("0", &v), fnd!("0", &v)],
            vec![fnd!("0", &v), fnd!("2", &v)]
        ]
    )
}
