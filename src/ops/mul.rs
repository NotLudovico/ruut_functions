use std::ops::Mul;

use crate::{simp::simp_node, Func};

impl Mul for Func {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self == 1 {
            return rhs;
        }
        if rhs == 1 {
            return self;
        }
        if self == 0 || rhs == 0 {
            return Func::Num(0);
        }

        let mut func = match (&self, &rhs) {
            (Func::Num(mul1), Func::Num(mul2)) => Func::Num(mul1 * mul2),
            (Func::Mul(mul1), Func::Mul(mul2)) => Func::Mul([&mul1[..], &mul2[..]].concat()),
            (Func::Mul(mul), other) | (other, Func::Mul(mul)) => {
                Func::Mul([&mul[..], &[other.clone()]].concat())
            }
            (_, _) => Func::Mul(vec![self, rhs]),
        };
        simp_node(&mut func);
        func
    }
}
impl Mul<Func> for i32 {
    type Output = Func;

    fn mul(self, rhs: Func) -> Self::Output {
        Func::Num(self) * rhs
    }
}
