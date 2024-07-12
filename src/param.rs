use crate::{Func, F1D, F2D, F3D};

impl Func {
    fn set_par(&mut self, name: &str, val: f64) {
        match self {
            Func::Param(c, v) if c == name => {
                *v = val;
            }
            Func::Add(arr) | Func::Mul(arr) => {
                for el in arr {
                    el.set_par(name, val)
                }
            }
            Func::Pow(base, exp) => {
                base.set_par(name, val);
                exp.set_par(name, val);
            }
            Func::S(_, arg) => arg.set_par(name, val),
            _ => (),
        }
    }
}

impl F1D {
    /// Set param value by name
    /// ```
    /// use ruut_functions::{f1d, F1D};
    ///
    /// let mut f = f1d!("x+[a]^2");
    /// f.set_par("a", 6.9);
    /// assert!(f.eval(4.20) - 51.81 < 0.0001);
    /// f.set_par("a", 1.2);
    /// assert!(f.eval(4.20) - 5.64 < 0.0001);
    /// ```
    pub fn set_par(&mut self, name: &str, val: f64) {
        self.0.set_par(name, val)
    }
}
impl F2D {
    /// Set param value by name
    /// ```
    /// use ruut_functions::{f2d, F2D};
    ///
    /// let mut f = f2d!("x+y[a]^2");
    /// f.set_par("a", 6.9);
    /// assert!(f.eval(4.20, 1.0) - 51.81 < 0.0001);
    /// f.set_par("a", 1.2);
    /// assert!(f.eval(4.20, 1.0) - 5.64 < 0.0001);
    /// ```
    pub fn set_par(&mut self, name: &str, val: f64) {
        self.0.set_par(name, val)
    }
}
impl F3D {
    /// Set param value by name
    /// ```
    /// use ruut_functions::{f3d, F3D};
    ///
    /// let mut f = f3d!("x+y[a]^2+z");
    /// f.set_par("a", 6.9);
    /// assert!(f.eval(4.20, 1.0,7.0) - 58.81 < 0.0001);
    /// f.set_par("a", 1.2);
    /// assert!(f.eval(4.20, 1.0, 7.0) - 12.64 < 0.0001);
    /// ```
    pub fn set_par(&mut self, name: &str, val: f64) {
        self.0.set_par(name, val)
    }
}
