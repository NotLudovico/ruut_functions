use crate::{FType, Func, F1D, F2D, F3D, FND};
use std::f64;

impl F1D {
    /// Evaluates function at x
    pub fn eval(&self, x: f64) -> f64 {
        self.0.eval(&[('x', x)])
    }
}
impl F2D {
    /// Evaluates function at (x,y)
    pub fn eval(&self, x: f64, y: f64) -> f64 {
        self.0.eval(&[('x', x), ('y', y)])
    }
}
impl F3D {
    /// Evaluates function at (x,y,z)
    pub fn eval(&self, x: f64, y: f64, z: f64) -> f64 {
        self.0.eval(&[('x', x), ('y', y), ('z', z)])
    }
}
impl FND {
    /// Evaluates function
    pub fn eval(&self, val: &[(char, f64)]) -> f64 {
        self.func.eval(val)
    }
}

impl Func {
    fn eval(&self, val: &[(char, f64)]) -> f64 {
        match &self {
            Func::Var(var) => val.iter().find(|&x| x.0 == *var).unwrap().1,
            Func::Num(val) => *val as f64,
            Func::E => std::f64::consts::E,
            Func::PI => std::f64::consts::PI,
            Func::Param(_, v) => *v,
            Func::Add(add) => add.iter().map(|term| term.eval(val)).sum::<f64>(),
            Func::Mul(mul) => mul.iter().map(|term| term.eval(val)).product::<f64>(),
            Func::Pow(base, exp) => base.eval(val).powf(exp.eval(val)),
            Func::S(kind, arg) => {
                let arg = arg.eval(val);
                match kind {
                    FType::Ln => arg.ln(),
                    FType::Sin => arg.sin(),
                    FType::Cos => arg.cos(),
                    FType::Tan => arg.tan(),
                    FType::Cot => 1. / arg.tan(),
                    FType::Sec => 1. / arg.cos(),
                    FType::Csc => 1. / arg.sin(),
                    FType::ASin => arg.asin(),
                    FType::ACos => arg.acos(),
                    FType::ATan => arg.atan(),
                    FType::Sinh => arg.sinh(),
                    FType::Cosh => arg.cosh(),
                    FType::Tanh => arg.tanh(),
                    FType::Coth => 1. / arg.tanh(),
                    FType::Sech => 1. / arg.cosh(),
                    FType::Csch => 1. / arg.sinh(),
                    FType::ASinh => arg.asinh(),
                    FType::ACosh => arg.acosh(),
                    FType::ATanh => arg.atanh(),
                    FType::Abs => arg.abs(),
                }
            }
        }
    }
}

/// Evaluates an array of F1D at x
pub fn eval_vec_f1d(vec: &[F1D], x: f64) -> Vec<f64> {
    let mut result = Vec::new();
    for el in vec {
        result.push(el.eval(x))
    }
    result
}
/// Evaluates an array of F2D at (x,y)
pub fn eval_vec_f2d(vec: &[F2D], x: f64, y: f64) -> Vec<f64> {
    let mut result = Vec::new();
    for el in vec {
        result.push(el.eval(x, y))
    }
    result
}
/// Evaluates an array of F3D at (x,y,z)
pub fn eval_vec_f3d(vec: &[F3D], x: f64, y: f64, z: f64) -> Vec<f64> {
    let mut result = Vec::new();
    for el in vec {
        result.push(el.eval(x, y, z))
    }
    result
}
#[test]
fn test_eval() {
    use crate::{f1d, f2d, f3d, fnd};
    assert_eq!(f1d!("(2/3)-(1/3)x").eval(1.), 0.3333333333333333);
    assert_eq!(f1d!("1/x").eval(0.), f64::INFINITY);
    assert_eq!(f2d!("xy+sin(x)").eval(3., 5.), 15.141120008059866);
    assert_eq!(f3d!("xyz*e*pi+1-x").eval(3., 5., 7.), 894.6720933807245);
    assert_eq!(
        fnd!("f+g^2", &['f', 'g']).eval(&[('f', 6.), ('g', 7.)]),
        55.
    );

    // vecs
    assert_eq!(
        eval_vec_f1d(&vec![f1d!("cos(x)"), f1d!("tan(x)sec(x)")], 2.),
        vec![-0.4161468365471424, 5.25064633769958]
    );

    assert_eq!(
        eval_vec_f2d(
            &vec![f2d!("cot(xy)+csc(y)"), f2d!("asin(y)+acos(y)")],
            2.,
            1.
        ),
        vec![0.7307375514178355, f64::consts::FRAC_PI_2]
    );

    assert_eq!(
        eval_vec_f3d(
            &[
                f3d!("atan(x)+sinh(y)+cosh(z)+tanh(xy)"),
                f3d!("coth(x)+sech(y)+csch(z)+asinh(x)+acosh(x)+atanh(y)+abs(x)")
            ],
            1.,
            0.5,
            0.2
        ),
        vec![2.788677381770281, 9.59735546963752]
    );
}
