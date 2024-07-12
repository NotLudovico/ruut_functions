use ruut_functions::{eval_vec_f2d, f2d, F2D, F3D};

fn main() {
    let mut func = f2d!("x+y^2[f]");
    func.set_par("f", 69.);
    let grad = func.gradient();
    let grad_at_point = eval_vec_f2d(&grad, 1., 1.);
    let func_3 = F3D::from(func);
}
