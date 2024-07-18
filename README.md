# Features
- 1D, 2D, 3D, ND (with custom variables) functions
- Set params and assign values (default is 0.0) 
- Compute derivatives of any orders, gradients, hessian
- F1D can be numerically integrated
- Supports the following functions:
    - Ln, Sin, Cos, Tan, Sec, Csc, ASin, ACos, ATan, Sinh, Cosh, Tanh, Coth, Sech, Csch, ASinh, ACosh, ATanh, Abs
- Some kind of expression semplification

# Examples
```rust
use ruut_functions::{f3d, F3D};

fn main() {
    let mut f = f3d!("x^3+y^2+yx^2+[eta]");
    f.set_par("eta", 6.9);
    assert_eq!(f.eval(0., 0., 0.), 6.9);
    assert_eq!(
        f.gradient(),
        vec![f3d!("2xy+3x^2"), f3d!("2y+x^2"), f3d!("0")]
    );
    assert_eq!(
        f.hessian(),
        vec![
            vec![f3d!("2y+6x"), f3d!("2x"), f3d!("0")],
            vec![f3d!("2x"), f3d!("2"), f3d!("0")],
            vec![f3d!("0"), f3d!("0"), f3d!("0")]
        ]
    )
}
```
