use crate::{FType, Func};

impl Func {
    pub(crate) fn pow(self, exp: Self) -> Self {
        if exp == 1 {
            return self;
        }
        if exp == 0 {
            return Func::Num(1);
        }

        if let Func::E = self {
            if let Func::S(FType::Ln, arg) = exp {
                return *arg.clone();
            }
        }

        Func::Pow(Box::new(self), Box::new(exp))
    }

    pub(crate) fn powi(self, exp: i32) -> Self {
        if exp == 0 {
            return Func::Num(1);
        }
        if exp == 1 {
            return self;
        }

        Func::Pow(Box::new(self), Box::new(Func::Num(exp)))
    }
}
