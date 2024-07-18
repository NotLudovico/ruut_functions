pub mod builder;

use std::collections::VecDeque;

use crate::FType;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Grammar {
    Var(char),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    E,
    PI,
    LPar,
    Num(i32),
    Param(String),
    S(FType),
    Sqrt,
}
pub(crate) fn to_rpn(input: &str, vars: &[char]) -> Result<VecDeque<Grammar>, ParsingError> {
    let mut curr = String::new();
    let mut chars = input.chars().peekable();

    let mut output_queue: VecDeque<Grammar> = VecDeque::new();
    let mut operator_stack: Vec<Grammar> = Vec::new();
    let mut prev = None;
    let mut found_param = false;

    while let Some(char) = chars.next() {
        if char == ' ' {
            continue;
        }
        curr.push(char);
        let next = chars.peek();

        if curr.parse::<i32>().is_ok() {
            if let Some(char) = next {
                if !char.is_ascii_digit() {
                    output_queue.push_back(Grammar::Num(curr.parse::<i32>().unwrap()));
                    implicit_mul(&next, &mut operator_stack, &mut output_queue);
                    curr.clear();
                }
            } else {
                output_queue.push_back(Grammar::Num(curr.parse::<i32>().unwrap()));
            }
        } else if curr == "e" {
            output_queue.push_back(Grammar::E);
            curr.clear();
        } else if curr == "pi" {
            output_queue.push_back(Grammar::PI);
            implicit_mul(&next, &mut operator_stack, &mut output_queue);
            curr.clear();
        } else if curr == "[" {
            found_param = true;
        } else if char == ']' {
            output_queue.push_back(Grammar::Param(curr[1..(curr.len() - 1)].to_string()));
            found_param = false;
            implicit_mul(&next, &mut operator_stack, &mut output_queue);
            curr.clear();
        } else if curr.len() == 1 && vars.contains(&curr.chars().next().unwrap()) {
            output_queue.push_back(Grammar::Var(char));
            curr.clear();
            implicit_mul(&next, &mut operator_stack, &mut output_queue);
        } else if !found_param
            && (match_operator(char, &next, &prev, &mut operator_stack, &mut output_queue)?
                || match_func(&curr, &next, &mut operator_stack)?)
        {
            curr.clear();
        }

        prev = Some(char);
    }

    while !operator_stack.is_empty() {
        if let Grammar::LPar = operator_stack.last().unwrap() {
            return Err(ParsingError::NotMatchingPar);
        }
        output_queue.push_back(operator_stack.pop().unwrap());
    }

    Ok(output_queue)
}

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    NotMatchingPar,
    UnknownFunction(String),
}

fn implicit_mul(
    next: &Option<&char>,
    operator_stack: &mut Vec<Grammar>,
    output_queue: &mut VecDeque<Grammar>,
) {
    if let Some(n) = next {
        if !matches!(n, '+' | '-' | '/' | '*' | '^' | ' ' | ')') {
            let _ = match_operator('*', &None, &None, operator_stack, output_queue);
        }
    }
}

fn match_operator(
    curr: char,
    next: &Option<&char>,
    prev: &Option<char>,
    operator_stack: &mut Vec<Grammar>,
    output_queue: &mut VecDeque<Grammar>,
) -> Result<bool, ParsingError> {
    let o1 = match curr {
        '+' => Grammar::Add,
        '-' => {
            if let Some(p) = prev {
                if *p == '(' {
                    output_queue.push_back(Grammar::Num(-1));
                    Grammar::Mul
                } else {
                    Grammar::Sub
                }
            } else {
                output_queue.push_back(Grammar::Num(-1));
                Grammar::Mul
            }
        }
        '*' => Grammar::Mul,
        '/' => Grammar::Div,
        '^' => Grammar::Pow,
        '(' => {
            operator_stack.push(Grammar::LPar);
            return Ok(true);
        }
        ')' => {
            while !operator_stack.is_empty() {
                match operator_stack.last() {
                    Some(o2) => {
                        if let Grammar::LPar = o2 {
                            break;
                        }
                        output_queue.push_back(operator_stack.pop().unwrap());
                    }
                    _ => break,
                }
            }

            if let Some(Grammar::LPar) = operator_stack.last() {
                operator_stack.pop();
            } else {
                return Err(ParsingError::NotMatchingPar);
            }
            implicit_mul(next, operator_stack, output_queue);
            return Ok(true);
        }

        _ => return Ok(false),
    };

    while !operator_stack.is_empty() {
        match operator_stack.last() {
            Some(o2) => {
                if let Grammar::LPar = o2 {
                    break;
                }

                if op_prec(o2).0 > op_prec(&o1).0
                    || (op_prec(o2).0 == op_prec(&o1).0 && op_prec(&o1).1)
                {
                    output_queue.push_back(operator_stack.pop().unwrap());
                } else {
                    break;
                }
            }
            _ => break,
        }
    }
    operator_stack.push(o1);

    Ok(true)
}

fn match_func(
    curr: &str,
    next: &Option<&char>,
    operator_stack: &mut Vec<Grammar>,
) -> Result<bool, ParsingError> {
    if let Some(char) = next {
        if **char == '(' {
            operator_stack.push(match curr {
                "sin" => Grammar::S(FType::Sin),
                "cos" => Grammar::S(FType::Cos),
                "tan" => Grammar::S(FType::Tan),
                "cot" => Grammar::S(FType::Cot),
                "sec" => Grammar::S(FType::Sec),
                "csc" => Grammar::S(FType::Csc),
                "asin" => Grammar::S(FType::ASin),
                "acos" => Grammar::S(FType::ACos),
                "atan" => Grammar::S(FType::ATan),
                "sinh" => Grammar::S(FType::Sinh),
                "cosh" => Grammar::S(FType::Cosh),
                "tanh" => Grammar::S(FType::Tanh),
                "coth" => Grammar::S(FType::Coth),
                "sech" => Grammar::S(FType::Sech),
                "csch" => Grammar::S(FType::Csch),
                "asinh" => Grammar::S(FType::ASinh),
                "acosh" => Grammar::S(FType::ACosh),
                "atanh" => Grammar::S(FType::ATanh),
                "abs" => Grammar::S(FType::Abs),
                "ln" | "log" => Grammar::S(FType::Ln),
                "sqrt" => Grammar::Sqrt,
                _ => return Err(ParsingError::UnknownFunction(curr.to_string())),
            });
            return Ok(true);
        }
    } else {
        return Err(ParsingError::UnknownFunction(curr.to_string()));
    }

    Ok(false)
}

// Returns if it's left-associative
fn op_prec(op: &Grammar) -> (usize, bool) {
    match op {
        Grammar::Add | Grammar::Sub => (2, true),
        Grammar::Mul | Grammar::Div => (3, true),
        Grammar::Pow => (4, false),
        _ => (5, true),
    }
}

#[test]
fn test_to_rpn() {
    assert_eq!(
        to_rpn("sin(x)/cos(-x)", &['x']).unwrap(),
        VecDeque::from([
            Grammar::Var('x'),
            Grammar::S(FType::Sin),
            Grammar::Num(-1),
            Grammar::Var('x'),
            Grammar::Mul,
            Grammar::S(FType::Cos),
            Grammar::Div,
        ])
    );
    assert_eq!(
        to_rpn("-13/(15+7^3)*sinh(69)+x+e^x+pi", &['x']).unwrap(),
        VecDeque::from([
            Grammar::Num(-1),
            Grammar::Num(13),
            Grammar::Mul,
            Grammar::Num(15),
            Grammar::Num(7),
            Grammar::Num(3),
            Grammar::Pow,
            Grammar::Add,
            Grammar::Div,
            Grammar::Num(69),
            Grammar::S(FType::Sinh),
            Grammar::Mul,
            Grammar::Var('x'),
            Grammar::Add,
            Grammar::E,
            Grammar::Var('x'),
            Grammar::Pow,
            Grammar::Add,
            Grammar::PI,
            Grammar::Add
        ])
    );

    assert_eq!(
        to_rpn("3*(2-))7", &['x']).unwrap_err(),
        ParsingError::NotMatchingPar
    );
    assert_eq!(
        to_rpn("3*2((7", &['x']).unwrap_err(),
        ParsingError::NotMatchingPar
    );
    assert_eq!(
        to_rpn("yomama(x)", &['x']).unwrap_err(),
        ParsingError::UnknownFunction("yomama".to_string())
    );

    // Implicit multiplication
    assert_eq!(
        to_rpn("x(x+1)[par]", &['x']).unwrap(),
        VecDeque::from([
            Grammar::Var('x'),
            Grammar::Var('x'),
            Grammar::Num(1),
            Grammar::Add,
            Grammar::Mul,
            Grammar::Param(String::from("par")),
            Grammar::Mul
        ])
    );
    // Implicit multiply
    assert_eq!(
        to_rpn(
            "x+sin(x)/cos(x)+ln(4*x)*ln(7)+sin(x^2)^2+tan(14*x)/cos(14*x)+3/2-1/7+cos(x)*cos(x)+sin(x)*sin(x)+cos(x)*cos(x)",
            &['x'],
        ).unwrap(),
        to_rpn(
            "x+sin(x)/cos(x)+ln(4x)ln(7)+sin(x^2)^2+tan(14x)/cos(14x)+3/2-1/7+cos(x)cos(x)+sin(x)sin(x)+cos(x)cos(x)",
            &['x'],
        ).unwrap()
    );

    // Power problem
    assert_eq!(
        to_rpn("e^(-[eta]xy)", &['x', 'y']).unwrap(),
        VecDeque::from([
            Grammar::E,
            Grammar::Num(-1),
            Grammar::Param(String::from("eta")),
            Grammar::Mul,
            Grammar::Var('x'),
            Grammar::Mul,
            Grammar::Var('y'),
            Grammar::Mul,
            Grammar::Pow
        ])
    )
}
