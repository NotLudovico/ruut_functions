#[macro_export]
/// Generate F1D from string and unwrap the result
macro_rules! f1d {
    ($expr: literal) => {
        F1D::new($expr).unwrap()
    };
}
#[macro_export]
/// Generate F2D from string and unwrap the result
macro_rules! f2d {
    ($expr: literal) => {
        F2D::new($expr).unwrap()
    };
}
#[macro_export]
/// Generated F3D from string and unwrap the resul
macro_rules! f3d {
    ($expr: literal) => {
        F3D::new($expr).unwrap()
    };
}

#[macro_export]
/// Generate F3D from string and unwrap the resul
macro_rules! fnd {
    ($expr: literal, $vars: expr) => {
        FND::new($expr, $vars).unwrap()
    };
}
