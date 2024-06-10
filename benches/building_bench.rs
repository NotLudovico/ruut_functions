use criterion::{criterion_group, criterion_main, Criterion};
use ruut_functions::F1D;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("easy as fuck", |b| b.iter(|| F1D::new("(sin(3+7)/8)+7^2")));
    c.bench_function("Creating F1D", |b| {
        b.iter(|| F1D::new("x+sin(x)/cos(x)+ln(4x)ln(7)+sin(x^2)^2+tan(14x)/cos(14x)+3/2-1/7"))
    });
    // c.bench_function("create F1D", |b| {
    //     b.iter(|| f1d!("x+sin(x)/cos(x)+ln(4x)ln(7)+sin(x^2)^2+tan(14x)/cos(14x)+3/2-1/7"))
    // });
    // c.bench_function("Derivative of F1D", |b| {
    //     b.iter(|| {
    //         f1d!("x+sin(x)/cos(x)+ln(4x)ln(7)+sin(x^2)^2+tan(14x)/cos(14x)+3/2-1/7").derivative()
    //     })
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
