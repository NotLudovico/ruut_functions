use std::fmt::Display;

use crate::{FType, Func, F1D};

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::E => write!(f, "e"),
            Self::PI => write!(f, "\u{1D70B}"),
            Self::Var(char) => write!(f, "{}", char),
            Self::Num(num) => write!(f, "{}", num),
            Self::Param(par) => write!(f, "[{}]", par),
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

                for el in mul {
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
                                    output += &format!("/({}", base);
                                } else {
                                    output += &format!("{}", base);
                                }

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
                if found_div {
                    write!(f, "{})", output)
                } else {
                    write!(f, "{}", output)
                }
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

impl<'a> Display for F1D<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.func)
    }
}

impl Func {
    pub fn latex(&self) -> String {
        match self {
            Func::Var(char) => char.to_string(),
            Func::PI => String::from("\\pi"),
            Func::E => String::from("e"),
            Func::Num(val) => {
                if *val == -1 {
                    "-".to_string()
                } else {
                    format!("{}", val)
                }
            }
            Func::Param(par) => format!("\text{{{par}}}"),
            Func::Add(add) => {
                let mut output = String::from("");

                for (i, el) in add.iter().enumerate() {
                    if i != 0 {
                        output += &format!("+{}", &el.latex());
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
                                output = format!("\\frac{{{}}}{{", output);
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
                    _ => output += &format!("{}", base.latex()),
                };
                match **exp {
                    Func::Add(_) | Func::Mul(_) | Func::Pow(..) => {
                        output += &format!("^{{{}}}", exp.latex())
                    }
                    Func::Num(val) if val == 1 => (),
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
    assert_eq!(
        format!(
            "{}",
            F1D::new("x+1+cos(x)/ln(x)/ln(15)*sinh(x)^2/7-2").unwrap()
        ),
        "-1+x+cos(x)sinh(x)^2/(7ln(x)ln(15))",
    )
}

#[test]
fn test_latex() {
    use crate::F3D;
    assert_eq!(
        format!("{}", F3D::new("x^(y^3)+sin(x)+e+pi").unwrap().latex()),
        "\\pi+e+sin(x)+x^{y^3}"
    );

    assert_eq!(
        format!("{}", F3D::new("z+3ln(x)/(xy)").unwrap().latex()),
        "z+\\frac{3ln(x)}{xy}"
    );

    // let ctx = Ctx::new(vec![("eta", 0.45)]);
    // let func = F3D::build("eta^2+cos(x)-tan(x)", &ctx).unwrap();
    // assert_eq!(func.latex(), "-tan(x)+\text{eta}^2+cos(x)");

    assert_eq!(F3D::new("(x+2)^3").unwrap().latex(), "(2+x)^3");
    assert_eq!(F3D::new("(x+2)^(3+y)").unwrap().latex(), "(2+x)^{3+y}");
    assert_eq!(F3D::new("x^(3+y)").unwrap().latex(), "x^{3+y}");
}
