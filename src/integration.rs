use crate::F1D;

impl F1D {
    /// Computes the definite integral of F1D
    /// ```
    /// use ruut_functions::{F1D,f1d};
    ///
    /// let func = f1d!("x^2+6");
    ///
    /// assert!(func.integrate(0.,1., 10_000) - 6.33333 < 0.00001)
    /// ```
    pub fn integrate(&self, a: f64, b: f64, steps: u32) -> f64 {
        let mut result = 0.;

        for i in 1..=steps {
            // Evaluating Func at midpoint of dx
            result += self.eval(a + ((b - a) / steps as f64) * (i as f64 - 0.5));
        }

        ((b - a) / steps as f64) * result
    }
}

#[test]
fn test_integration() {
    use crate::f1d;
    let func = f1d!("x^3");
    assert!(func.integrate(-1., 1.5, 10_000) - 1.0156 < 0.0001);

    let func = f1d!("sin(x)^2");
    assert!(func.integrate(0., 2. * std::f64::consts::PI, 10_000) - std::f64::consts::PI < 0.00001);
}
