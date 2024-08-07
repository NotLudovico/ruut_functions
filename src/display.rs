use std::fmt::Display;

use crate::{FType, Func, F1D, F2D, F3D};

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::E => write!(f, "e"),
            Self::PI => write!(f, "\u{1D70B}"),
            Self::Var(char) => write!(f, "{}", char),
            Self::Num(num) => write!(f, "{}", num),
            Self::Param(par, _) => write!(f, "[{}]", par),
            Self::S(kind, arg) => match kind {
                FType::Sin => write!(f, "sin({})", arg),
                FType::Cos => write!(f, "cos({})", arg),
                FType::Tan => write!(f, "tan({})", arg),
                FType::Cot => write!(f, "cot({})", arg),
                FType::Sec => write!(f, "sec({})", arg),
                FType::Csc => write!(f, "csc({})", arg),
                FType::ASin => write!(f, "asin({})", arg),
                FType::ACos => write!(f, "acos({})", arg),
                FType::ATan => write!(f, "atan({})", arg),
                FType::Sinh => write!(f, "sinh({})", arg),
                FType::Cosh => write!(f, "cosh({})", arg),
                FType::Tanh => write!(f, "tanh({})", arg),
                FType::Coth => write!(f, "coth({})", arg),
                FType::Sech => write!(f, "sech({})", arg),
                FType::Csch => write!(f, "csch({})", arg),
                FType::ASinh => write!(f, "asinh({})", arg),
                FType::ACosh => write!(f, "acosh({})", arg),
                FType::ATanh => write!(f, "atanh({})", arg),
                FType::Abs => write!(f, "|{}|", arg),
                FType::Ln => write!(f, "ln({})", arg),
            },
            Self::Add(add) => {
                let mut output = String::from("");
                for (i, el) in add.iter().enumerate() {
                    if i != 0 {
                        if let Func::Mul(mul) = el {
                            if !mul.is_empty() {
                                if let Func::Num(val) = mul[0] {
                                    if val < 0 {
                                        output += &format!("{}", el);
                                        continue;
                                    }
                                }
                            }
                        }
                        output += &format!("+{}", el);
                    } else {
                        output += &format!("{}", el);
                    }
                }
                write!(f, "{}", output)
            }
            Self::Mul(mul) => {
                let mut output = String::from("");
                let mut found_div = false;
                let mut has_divs = false;

                for (i, el) in mul.iter().enumerate() {
                    if let Func::Num(val) = el {
                        if *val == -1 {
                            output += "-";
                            continue;
                        }
                    }
                    if let Func::Pow(base, exp) = el {
                        if let Func::Num(e) = **exp {
                            if e < 0 {
                                if !found_div {
                                    output += "/";
                                    if i != mul.len() - 1 {
                                        has_divs = true;
                                        output += "(";
                                    }
                                }

                                match **base {
                                    Func::Add(_) | Func::Mul(_) => output += &format!("({})", base),
                                    _ => output += &format!("{}", base),
                                };
                                if e != -1 {
                                    output += &format!("^{}", -e);
                                }

                                found_div = true;
                                continue;
                            }
                        }
                    }
                    output += &format!("{}", el);
                }
                if has_divs {
                    output += ")";
                }
                write!(f, "{}", output)
            }
            Func::Pow(base, exp) => {
                let mut output = String::new();

                match **base {
                    Func::Add(_) | Func::Mul(_) => output += &format!("({})^", base),
                    _ => output += &format!("{}^", base),
                };
                match **exp {
                    Func::Add(_) | Func::Mul(_) => output += &format!("({})", exp),
                    _ => output += &format!("{}", exp),
                };

                write!(f, "{}", output)
            }
        }
    }
}

impl Display for F1D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Display for F2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Display for F3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Func {
    pub(crate) fn latex(&self) -> String {
        match self {
            Func::Var(char) => char.to_string(),
            Func::PI => String::from(r"\pi"),
            Func::E => String::from("e"),
            Func::Num(val) => format!("{}", val),
            Func::Param(par, _) => format!(r"\text{{{par}}}"),
            Func::Add(add) => {
                let mut output = String::from("");

                for (i, el) in add.iter().enumerate() {
                    if i != 0 {
                        if let Func::Mul(mul) = el {
                            if !mul.is_empty() {
                                if let Func::Num(val) = mul[0] {
                                    if val < 0 {
                                        output += &format!("{}", el);
                                        continue;
                                    }
                                }
                            }
                        }
                        output += &format!("+{}", el.latex());
                    } else {
                        output += &el.latex();
                    }
                }
                output
            }
            Func::Mul(mul) => {
                let mut output = String::from("");
                let mut found_div = false;

                for el in mul {
                    if let Func::Pow(base, exp) = el {
                        if **exp < 0 {
                            if !found_div {
                                output = format!(r"\frac{{{}}}{{", output);
                            }
                            found_div = true;
                            let div = Func::Pow(base.clone(), Box::new(-1 * *exp.clone()));
                            output += &div.latex();
                            continue;
                        }
                    }

                    output += &el.latex();
                }

                if found_div {
                    output += "}";
                }
                output
            }
            Func::Pow(base, exp) => {
                let mut output = String::new();

                match **base {
                    Func::Add(_) | Func::Mul(_) | Func::Pow(..) => {
                        output += &format!("({})", base.latex())
                    }
                    _ => output += &base.latex(),
                };
                match **exp {
                    Func::Add(_) | Func::Mul(_) | Func::Pow(..) => {
                        output += &format!("^{{{}}}", exp.latex())
                    }
                    Func::Num(1) => (),
                    _ => output += &format!("^{}", exp.latex()),
                };

                output
            }
            Func::S(kind, arg) => match kind {
                FType::Ln => format!("ln({})", arg.latex()),
                FType::Sin => format!("sin({})", arg.latex()),
                FType::Cos => format!("cos({})", arg.latex()),
                FType::Tan => format!("tan({})", arg.latex()),
                FType::Cot => format!("cot({})", arg.latex()),
                FType::Sec => format!("sec({})", arg.latex()),
                FType::Csc => format!("csc({})", arg.latex()),
                FType::ACos => format!("acos({})", arg.latex()),
                FType::ASin => format!("asin({})", arg.latex()),
                FType::ATan => format!("atan({})", arg.latex()),
                FType::Sinh => format!("sinh({})", arg.latex()),
                FType::Cosh => format!("cosh({})", arg.latex()),
                FType::Tanh => format!("tanh({})", arg.latex()),
                FType::Coth => format!("coth({})", arg.latex()),
                FType::Sech => format!("sech({})", arg.latex()),
                FType::Csch => format!("csch({})", arg.latex()),
                FType::ASinh => format!("asinh({})", arg.latex()),
                FType::ACosh => format!("acosh({})", arg.latex()),
                FType::ATanh => format!("atanh({})", arg.latex()),
                FType::Abs => format!("|{}|", arg.latex()),
            },
        }
    }
}

#[test]
fn test_display() {
    use crate::{f1d, f2d, f3d};
    assert_eq!(
        format!("{}", f1d!("x+1+cos(x)/ln(x)/ln(15)*sinh(x)^2/7-2+e*pi")),
        "-1+x+cos(x)sinh(x)^2/(7ln(x)ln(15))+𝜋e",
    );

    assert_eq!(
        format!("{}", f1d!("[eta]*sin(x)+cosh(x)/tan(x^2)")),
        "[eta]sin(x)+cosh(x)/tan(x^2)"
    );
    assert_eq!(
        format!("{}", f1d!("cot(x)+sec(x)^2+csc(e)/acos(x)")),
        "csc(e)/acos(x)+cot(x)+sec(x)^2"
    );

    assert_eq!(format!("{}", f2d!("x+(-5y)")), "x-5y");
    assert_eq!(format!("{}", f2d!("-xy")), "-xy");
    assert_eq!(format!("{}", f2d!("x/(x+y)^2")), "x/(x+y)^2");
    assert_eq!(format!("{}", f2d!("(x+y)^(e+2)")), "(x+y)^(2+e)");
    assert_eq!(
        format!(
            "{}",
            f2d!(
                "asin(x)+atan(x)+tanh(x)+coth(x)+sech(x)+csch(x)+asinh(x)+acosh(x)+atanh(x)+abs(x)"
            )
        ),
        "|x|+asin(x)+atan(x)+tanh(x)+coth(x)+sech(x)+csch(x)+asinh(x)+acosh(x)+atanh(x)"
    );
    assert_eq!(format!("{}", f3d!("xyz")), "xyz");
}

#[test]
fn test_latex() {
    use crate::{f1d, f2d, f3d};

    assert_eq!(
        format!("{}", f3d!("x^(y^3)+sin(x)+e+pi").latex()),
        r"\pi+e+sin(x)+x^{y^3}"
    );

    assert_eq!(
        format!("{}", f3d!("z+3ln(x)/(xy)").latex()),
        r"z+\frac{3ln(x)}{xy}"
    );

    assert_eq!(f3d!("(x+2)^3").latex(), "(2+x)^3");
    assert_eq!(f3d!("(x+2)^(3+y)").latex(), "(2+x)^{3+y}");
    assert_eq!(f3d!("x^(3+y)").latex(), "x^{3+y}");
    assert_eq!(f3d!("-x+[eta]").latex(), r"\text{eta}-x");
    assert_eq!(
        format!(
            "{}",
            f2d!(
                "asin(x)+atan(x)+tanh(x)+coth(x)+sech(x)+csch(x)+asinh(x)+acosh(x)+atanh(x)+abs(x)"
            )
            .latex()
        ),
        "|x|+asin(x)+atan(x)+tanh(x)+coth(x)+sech(x)+csch(x)+asinh(x)+acosh(x)+atanh(x)"
    );

    assert_eq!(
        format!(
            "{}",
            f1d!("cot(x)+sec(x)^2+csc(e)/acos(x)+cos(x)+tan(x)+sinh(x)cosh(x)").latex()
        ),
        r"\frac{csc(e)}{acos(x)}+sinh(x)cosh(x)+cos(x)+tan(x)+cot(x)+sec(x)^2"
    );
}
