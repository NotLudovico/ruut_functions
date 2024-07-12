use crate::{F1D, F2D, F3D};
use std::ops::{Add, Div, Mul, Sub};
mod add;
mod div;
mod mul;
mod pow;

// Cross operations
macro_rules! cross_ops {
    ($t1:ty, $t2: ty) => {
        impl Add<$t1> for $t2 {
            type Output = Self;
            fn add(self, rhs: $t1) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }
        impl Sub<$t1> for $t2 {
            type Output = Self;
            fn sub(self, rhs: $t1) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }
        impl Mul<$t1> for $t2 {
            type Output = Self;
            fn mul(self, rhs: $t1) -> Self::Output {
                Self(self.0 * rhs.0)
            }
        }
        impl Div<$t1> for $t2 {
            type Output = Self;
            fn div(self, rhs: $t1) -> Self::Output {
                Self(self.0 / rhs.0)
            }
        }
    };
}

cross_ops!(F1D, F2D);
cross_ops!(F1D, F3D);
cross_ops!(F2D, F3D);

#[test]
fn test_cross_ops() {
    use crate::{f1d, f2d};
    let f1 = f1d!("x");
    let f2 = f2d!("y");
    assert_eq!(f2 + f1, f2d!("x+y"));
}
