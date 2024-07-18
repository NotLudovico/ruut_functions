use std::collections::VecDeque;

use crate::Func;

use super::Grammar;

pub(crate) fn build(input: VecDeque<Grammar>) -> Func {
    let mut stack: Vec<Func> = Vec::with_capacity(4);
    for el in input {
        match el {
            Grammar::Num(val) => stack.push(Func::Num(val)),
            Grammar::Var(char) => stack.push(Func::Var(char)),
            Grammar::Param(name) => stack.push(Func::Param(name, 0.)),
            Grammar::Add => {
                let second = stack.pop().unwrap();

                *stack.last_mut().unwrap() += second;
            }
            Grammar::Sub => {
                let second = stack.pop().unwrap();
                let first = stack.pop();

                if let Some(el) = first {
                    stack.push(el - second);
                } else {
                    stack.push(-1 * second)
                }
            }
            Grammar::Mul => {
                let second = stack.pop().unwrap();

                *stack.last_mut().unwrap() *= second;
            }
            Grammar::Div => {
                let first = stack.pop().unwrap();
                let second = stack.pop().unwrap();
                stack.push(second / first);
            }
            Grammar::Pow => {
                let second = stack.pop().unwrap();
                let first = stack.pop().unwrap();
                stack.push(first.pow(second))
            }
            Grammar::E => stack.push(Func::E),
            Grammar::PI => stack.push(Func::PI),
            Grammar::S(kind) => {
                let arg = stack.pop().unwrap();
                stack.push(Func::S(kind, Box::new(arg)));
            }
            Grammar::Sqrt => {
                let arg = stack.pop().unwrap();
                stack.push(arg.pow(Func::Mul(vec![
                    Func::Num(1),
                    Func::Pow(Box::new(Func::Num(2)), Box::new(Func::Num(-1))),
                ])));
            }
            Grammar::LPar => panic!("Found left par in RPN representation"),
        }
    }

    stack.pop().unwrap()
}

#[test]
fn test_builder() {
    use super::to_rpn;
    use crate::FType;

    assert_eq!(
        build(to_rpn("cos(-x)+sqrt(x)+(x^(-3))^2", &['x']).unwrap()),
        build(to_rpn("cos(-x)+x^(1/2)+x^(-6)", &['x']).unwrap()),
    );

    let input = to_rpn("(sin(3+7)/8)-7^2", &['x']).unwrap();
    assert_eq!(
        build(input),
        Func::Add(vec![
            Func::Mul(vec![
                Func::S(FType::Sin, Box::new(Func::Num(10))),
                Func::Pow(Box::new(Func::Num(8)), Box::new(Func::Num(-1))),
            ]),
            Func::Mul(vec![
                Func::Num(-1),
                Func::Pow(Box::new(Func::Num(7)), Box::new(Func::Num(2)))
            ])
        ])
    );

    assert_eq!(
        build(to_rpn("e^(-[eta]xy)", &['x', 'y']).unwrap()),
        Func::Pow(
            Box::new(Func::E),
            Box::new(Func::Mul(vec![
                Func::Num(-1),
                Func::Param("eta".to_string(), 0.),
                Func::Var('x'),
                Func::Var('y')
            ]))
        )
    )
}
