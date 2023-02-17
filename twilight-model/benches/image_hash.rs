use criterion::{criterion_group, criterion_main, Criterion};
use twilight_model::util::image_hash::ImageHash;

fn parse_hash(hash: &str) {
    let _ = ImageHash::parse(hash.as_bytes()).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse non-animated hex", |b| {
        b.iter(|| parse_hash("e15db5d5ccaa4714a6fac0514d3d2037"));
    });
    c.bench_function("parse animated hex", |b| {
        b.iter(|| parse_hash("a_e15db5d5ccaa4714a6fac0514d3d2037"));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
