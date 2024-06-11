use crate::{FType, Func};

impl PartialEq<i32> for Func {
    fn eq(&self, other: &i32) -> bool {
        if let Func::Num(val) = self {
            return val == other;
        }

        false
    }
}

impl Eq for Func {}
impl PartialOrd for Func {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Func {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        func_order(self).cmp(&func_order(other))
    }
}

fn func_order(func: &Func) -> u32 {
    match func {
        Func::Num(_) => 0,
        Func::PI => 1,
        Func::E => 2,
        Func::Param(_) => 3,
        Func::Var(char) => char.to_ascii_lowercase() as u32,
        Func::Mul(_) => 123,
        Func::Add(_) => 124,
        Func::Pow(_, exp) => {
            if let Func::Num(val) = **exp {
                if val < 0 {
                    146
                } else {
                    125
                }
            } else {
                125
            }
        }
        Func::S(kind, _) => match kind {
            FType::Abs => 126,
            FType::Ln => 127,
            FType::Sin => 128,
            FType::Cos => 129,
            FType::Tan => 130,
            FType::Cot => 131,
            FType::Sec => 132,
            FType::Csc => 133,
            FType::ASin => 134,
            FType::ACos => 135,
            FType::ATan => 136,
            FType::Sinh => 137,
            FType::Cosh => 138,
            FType::Tanh => 139,
            FType::Coth => 140,
            FType::Sech => 141,
            FType::Csch => 142,
            FType::ASinh => 143,
            FType::ACosh => 144,
            FType::ATanh => 145,
        },
    }
}

impl std::iter::Sum for Func {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Func::Num(0), |acc, func| acc + func)
    }
}

impl std::iter::Product for Func {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Func::Num(1), |acc, func| acc * func)
    }
}
