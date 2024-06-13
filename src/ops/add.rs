use std::ops::{Add, AddAssign, Sub};

use crate::{simp::simp_node, Func};

impl Add for Func {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self == 0 {
            return rhs;
        }
        if rhs == 0 {
            return self;
        }

        let mut func = match (&self, &rhs) {
            (Func::Num(val), Func::Num(val2)) => Func::Num(val + val2),
            (Func::Add(add1), Func::Add(add2)) => Func::Add([&add1[..], &add2[..]].concat()),
            (Func::Add(add1), other) | (other, Func::Add(add1)) => {
                Func::Add([&add1[..], &[other.clone()]].concat())
            }
            (_, _) => Func::Add(vec![self, rhs]),
        };

        simp_node(&mut func);
        func
    }
}

impl AddAssign for Func {
    fn add_assign(&mut self, rhs: Self) {
        match (&mut *self, &rhs) {
            (Func::Num(val), Func::Num(val2)) => *val += val2,
            (Func::Add(add_lhs), Func::Add(add_rhs)) => {
                for el in add_rhs {
                    add_lhs.push(el.clone());
                }
            }
            (Func::Add(add), other) => {
                add.push(other.clone());
            }
            (_, _) => *self = Func::Add(vec![self.clone(), rhs]),
        };

        simp_node(self)
    }
}

impl Add<Func> for i32 {
    type Output = Func;

    fn add(self, rhs: Func) -> Self::Output {
        Func::Num(self) + rhs
    }
}

impl Sub for Func {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -1 * rhs
    }
}

impl Sub<i32> for Func {
    type Output = Func;

    fn sub(self, rhs: i32) -> Self::Output {
        self - Func::Num(rhs)
    }
}

impl Sub<Func> for i32 {
    type Output = Func;

    fn sub(self, rhs: Func) -> Self::Output {
        Func::Num(self) - rhs
    }
}
