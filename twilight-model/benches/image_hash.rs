use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use twilight_model::util::image_hash::ImageHash;

fn parse_hash(hash: &str) {
    let _ = ImageHash::parse(hash.as_bytes()).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    const ANIMATED: &str = "a_e15db5d5ccaa4714a6fac0514d3d2037";
    const NON_ANIMATED: &str = "e15db5d5ccaa4714a6fac0514d3d2037";

    c.bench_with_input(
        BenchmarkId::new("parse non-animated hex", NON_ANIMATED),
        &NON_ANIMATED,
        |bencher, input| {
            bencher.iter(|| parse_hash(input));
        },
    );
    c.bench_with_input(
        BenchmarkId::new("parse animated hex", ANIMATED),
        &ANIMATED,
        |bencher, input| {
            bencher.iter(|| parse_hash(input));
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
