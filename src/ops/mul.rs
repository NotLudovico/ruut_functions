use std::ops::{Mul, MulAssign};

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

impl MulAssign for Func {
    fn mul_assign(&mut self, rhs: Self) {
        match (&mut *self, &rhs) {
            (Func::Num(val), Func::Num(val2)) => *val *= val2,
            (Func::Mul(mul_lhs), Func::Mul(mul_rhs)) => {
                for el in mul_rhs {
                    mul_lhs.push(el.clone())
                }
            }
            (Func::Mul(mul), _) => {
                mul.push(rhs);
            }
            (_, _) => {
                if *self == 0 {
                    return;
                } else if *self == 1 {
                    *self = rhs.clone();
                } else {
                    *self = Func::Mul(vec![self.clone(), rhs]);
                }
            }
        }

        simp_node(self)
    }
}
impl Mul<Func> for i32 {
    type Output = Func;

    fn mul(self, rhs: Func) -> Self::Output {
        Func::Num(self) * rhs
    }
}
