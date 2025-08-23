use criterion::{Criterion, criterion_group, criterion_main};
use http::header::{HeaderMap, HeaderName as HttpHeaderName, HeaderValue};
use twilight_http_ratelimiting::headers::RatelimitHeaders;

fn global_header_iter(map: &HeaderMap) {
    let iter = map.iter().map(|(k, v)| (k.as_str(), v.as_bytes()));

    RatelimitHeaders::from_pairs(iter).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("global header iter", |b| {
        let mut map = HeaderMap::new();
        map.insert(
            HttpHeaderName::from_static("x-ratelimit-global"),
            HeaderValue::from_static("true"),
        );
        map.insert(
            HttpHeaderName::from_static("retry-after"),
            HeaderValue::from_static("65"),
        );

        b.iter(|| global_header_iter(&map))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
