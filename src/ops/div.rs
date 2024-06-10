use std::ops::Div;

use crate::{simp::simp_node, Func};

impl Div for Func {
    type Output = Self;

    fn div(self, div: Self) -> Self::Output {
        if div == 1 {
            return self;
        }

        if div == 0 {
            panic!("Dividing by 0")
        }

        if self == div {
            return Func::Num(1);
        }
        if self == 0 {
            return self;
        }

        let mut func = match (&self, &div) {
            (Func::Mul(mul), Func::Mul(div)) => Func::Mul(
                [
                    &mul[..],
                    &div.iter()
                        .map(|el| el.clone().powi(-1))
                        .collect::<Vec<Func>>()[..],
                ]
                .concat(),
            ),
            (Func::Mul(mul), other) => Func::Mul([&mul[..], &[other.clone().powi(-1)]].concat()),
            (other, Func::Mul(div)) => Func::Mul(
                [
                    &[other.clone()],
                    &div.iter()
                        .map(|el| el.clone().powi(-1))
                        .collect::<Vec<Func>>()[..],
                ]
                .concat(),
            ),
            (_, _) => Func::Mul(vec![self, div.powi(-1)]),
        };
        simp_node(&mut func);
        func
    }
}

impl Div<i32> for Func {
    type Output = Func;

    fn div(self, rhs: i32) -> Self::Output {
        self / Func::Num(rhs)
    }
}
