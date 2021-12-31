use criterion::{criterion_group, criterion_main, Criterion};
use twilight_model::util::image_hash::ImageHash;

const HASHES: &[&str] = &[
    "e15db5d5ccaa4714a6fac0514d3d2037",
    "d8994c0d9a8634482e15f82638e917ac",
    "6ac1667daa5cfec8409fff252db3fa18",
    "e46b13319befa1552527d8f472cc3144",
    "dcbcfe036f475287faabba053c21125a",
];

fn parse_hash(hash: &str) {
    let _ = ImageHash::parse(hash.as_bytes()).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("imagehash parse", |b| {
        b.iter(|| {
            for hash in HASHES {
                parse_hash(hash);
            }
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
