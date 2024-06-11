use ruut_algebra::gcd;

use crate::{FType, Func};

pub(crate) fn simp_node(func: &mut Func) {
    unwrap_par(func);

    match func {
        Func::Add(add) => {
            add.sort_unstable();
            if simp_add(add) {
                simp_node(func)
            }
        }
        Func::Mul(mul) => {
            mul.sort_unstable();
            if simp_mul(mul) {
                simp_node(func);
            }
        }
        Func::Pow(base, exp) => {
            if let Func::Pow(base_b, exp_b) = &**base {
                *func = Func::Pow(
                    Box::new(*base_b.clone()),
                    Box::new(*exp_b.clone() * *exp.clone()),
                );
            } else {
                simp_node(base);
                simp_node(exp);
            }
        }
        Func::S(FType::Ln, arg) => {
            if let Func::Pow(base, exp) = &**arg {
                if let Func::E = **base {
                    *func = *exp.clone()
                }
            } else {
                simp_node(arg)
            }
        }
        Func::S(FType::Cos, arg) => {
            if let Func::Mul(mul_vec) = &mut **arg {
                if mul_vec.len() > 1 {
                    if let Func::Num(val) = mul_vec[0] {
                        if val < 0 {
                            mul_vec[0] = Func::Num(-val);
                            simp_node(arg);
                        }
                    }
                }
            }
        }
        Func::S(_, arg) => simp_node(arg),
        _ => (),
    }
}

fn simp_add(add: &mut Vec<Func>) -> bool {
    let mut worked = false;

    for i in 0..add.len() {
        simp_node(&mut add[i]);

        let (firsts, others) = add.split_at_mut(i + 1);

        for second in others.iter_mut() {
            let new_func = match (&firsts[i], &*second) {
                (Func::Num(add1), Func::Num(add2)) => {
                    if *add1 != 0 && *add2 != 0 {
                        Some(Func::Num(add1 + add2))
                    } else {
                        None
                    }
                }
                (Func::S(FType::Ln, arg1), Func::S(FType::Ln, arg2)) => {
                    Some(Func::S(FType::Ln, Box::new(*arg1.clone() * *arg2.clone())))
                }
                (Func::Mul(lhs), Func::Mul(rhs)) => {
                    let mut lhs_c = (0, 1); // (index + 1) and value of coefficient
                    let mut rhs_c = (0, 1);

                    if let Func::Num(val) = lhs[0] {
                        lhs_c = (1, val);
                    }
                    if let Func::Num(val) = rhs[0] {
                        rhs_c = (1, val);
                    }

                    if lhs[lhs_c.0..lhs.len()] == rhs[rhs_c.0..rhs.len()] {
                        Some((lhs_c.1 + rhs_c.1) * Func::Mul(lhs[lhs_c.0..lhs.len()].to_vec()))
                    } else {
                        None
                    }
                }
                (Func::Pow(base, exp), Func::Pow(base2, exp2)) if **exp == 2 && **exp2 == 2 => {
                    match (&**base, &**base2) {
                        (Func::S(FType::Sin, arg1), Func::S(FType::Cos, arg2))
                        | (Func::S(FType::Cos, arg1), Func::S(FType::Sin, arg2))
                            if arg1 == arg2 =>
                        {
                            Some(Func::Num(1))
                        }
                        (_, _) => None,
                    }
                }
                (other, Func::Pow(base, exp)) | (Func::Pow(base, exp), other)
                    if *other == 1 && **exp == 2 =>
                {
                    let mut result = None;
                    if let Func::S(FType::Cot, arg) = &**base {
                        result = Some(Func::S(FType::Csc, arg.clone()).powi(2));
                    } else if let Func::S(FType::Tan, arg) = &**base {
                        result = Some(Func::S(FType::Sec, arg.clone()).powi(2));
                    }
                    result
                }
                (_, _) => None,
            };
            if let Some(f) = new_func {
                *second = f;
                firsts[i] = Func::Num(0);
                worked = true;
            } else if let Some(a) = is_rational(&firsts[i]) {
                if let Some(b) = is_rational(second) {
                    let num = a.0 * b.1 + b.0 * a.1;
                    let den = a.1 * b.1;
                    let gcd = gcd(num as u32, den as u32) as i32;
                    *second = Func::Mul(vec![Func::Num(num / gcd), Func::Num(den / gcd).powi(-1)]);
                    firsts[i] = Func::Num(0);
                    worked = true;
                }
            }
        }
    }

    // Remove zeros
    if remove_neutral(add, Func::Num(0)) {
        worked = true;
    };

    worked
}

fn simp_mul(mul: &mut Vec<Func>) -> bool {
    let mut worked = false;
    for i in 0..mul.len() {
        simp_node(&mut mul[i]);

        let (firsts, others) = mul.split_at_mut(i + 1);

        for second in others.iter_mut() {
            let new_func = match (&firsts[i], &second) {
                (Func::Num(mul1), Func::Num(mul2)) if *mul2 != 1 && *mul1 != 1 => {
                    Some(Func::Num(mul1 * mul2))
                }
                (Func::S(kind, arg1), Func::S(kind2, arg2)) if arg1 == arg2 => {
                    match (kind, kind2) {
                        (FType::Tan, FType::Cos) | (FType::Cos, FType::Tan) => {
                            Some(Func::S(FType::Sin, arg1.clone()))
                        }
                        (FType::Cot, FType::Sin) | (FType::Sin, FType::Cot) => {
                            Some(Func::S(FType::Cos, arg1.clone()))
                        }
                        (FType::Cot, FType::Tan)
                        | (FType::Tan, FType::Cot)
                        | (FType::Cos, FType::Sec)
                        | (FType::Sec, FType::Cos)
                        | (FType::Sin, FType::Csc)
                        | (FType::Csc, FType::Sin) => Some(Func::Num(1)),
                        (_, _) => None,
                    }
                }
                (Func::S(kind, arg1), Func::Pow(base, exp)) if **exp == -1 => {
                    let mut result = None;
                    if let Func::S(kind2, arg2) = &**base {
                        if arg1 == arg2 {
                            result = match (kind, kind2) {
                                (FType::Sin, FType::Cos) => Some(Func::S(FType::Tan, arg1.clone())),
                                (FType::Cos, FType::Sin) => Some(Func::S(FType::Cot, arg1.clone())),
                                (FType::Tan, FType::Sin) => Some(Func::S(FType::Sec, arg1.clone())),
                                (FType::Cot, FType::Cos) => Some(Func::S(FType::Csc, arg1.clone())),
                                (_, _) => None,
                            };
                        }
                    }
                    result
                }
                (other, Func::Pow(base, exp)) if **exp == -1 && *other == **base => {
                    Some(Func::Num(1))
                }
                (_, _) => None,
            };

            if let Some(f) = new_func {
                firsts[i] = Func::Num(1);
                *second = f;
                worked = true;
            } else if firsts[i] == *second {
                *second = firsts[i].clone().powi(2);
                firsts[i] = Func::Num(1);
                worked = true;
            }
        }
    }

    // Remove ones
    if remove_neutral(mul, Func::Num(1)) {
        worked = true
    };

    worked
}

fn remove_neutral(funcs: &mut Vec<Func>, neutral: Func) -> bool {
    let mut to_remove = vec![];
    let mut worked = false;
    for (i, el) in funcs.iter().enumerate() {
        if *el == neutral {
            to_remove.push(i);
        }
    }

    for i in to_remove.iter().rev() {
        if funcs.len() > 1 {
            funcs.remove(*i);
            worked = true;
        }
    }
    worked
}

pub(crate) fn is_rational(func: &Func) -> Option<(i32, i32)> {
    if let Func::Num(val) = func {
        return Some((*val, 1));
    }

    if let Func::Mul(mul) = func {
        if mul.len() == 2 {
            if let Func::Num(num) = mul[0] {
                if let Func::Pow(base, exp) = &mul[1] {
                    if let Func::Num(den) = **base {
                        if **exp == -1 {
                            let gcd = gcd(num.unsigned_abs(), den.unsigned_abs());
                            return Some((num / gcd as i32, den / gcd as i32));
                        }
                    }
                }
            }
        }
    }

    if let Func::Pow(base, exp) = func {
        if let Func::Num(den) = **base {
            if **exp == -1 {
                return Some((1, den));
            }
        }
    }

    None
}

fn unwrap_par(func: &mut Func) {
    match func {
        Func::Add(vec) | Func::Mul(vec) => {
            if vec.len() == 1 {
                *func = vec[0].clone();
            }
        }
        _ => (),
    }
}

#[test]
fn test_simp() {
    use crate::F1D;
    assert_eq!(
        F1D::new("sin(x)/cos(x)+1/2-7+sin(x)/cos(2x)").unwrap(),
        F1D::new("tan(x)-13/2+sin(x)/cos(2x)").unwrap()
    );

    assert_eq!(
        F1D::new("x+sin(x)/cos(-x)+ln(4x)+ln(7)+sin(x^2)^2+tan(14x)cos(14x)+3/2-1/7+cos(x)cos(x)+sin(x)sin(x)+2/28-10/7+cot(x)^2").unwrap(),
    F1D::new("x+tan(x)+ln(28x)+sin(x^2)^2+sin(14x)+csc(x)^2").unwrap()
    );
}
