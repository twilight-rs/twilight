use criterion::{criterion_group, criterion_main, Criterion};
use std::fmt::{Display, Write};
use twilight_mention::fmt::Mention;
use twilight_model::id::{ChannelId, EmojiId, RoleId, UserId};

fn format_id<T: Display>(input: &mut String, formatter: &T) {
    input.write_fmt(format_args!("{}", formatter)).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("format channel id", |b| {
        let mut string = String::new();
        let formatter = ChannelId::new(999_999_999_999_999_999)
            .expect("non zero")
            .mention();

        b.iter(|| format_id(&mut string, &formatter))
    });
    c.bench_function("format emoji id", |b| {
        let mut string = String::new();
        let formatter = EmojiId::new(999_999_999_999_999_999)
            .expect("non zero")
            .mention();

        b.iter(|| format_id(&mut string, &formatter))
    });
    c.bench_function("format role id", |b| {
        let mut string = String::new();
        let formatter = RoleId::new(999_999_999_999_999_999)
            .expect("non zero")
            .mention();

        b.iter(|| format_id(&mut string, &formatter))
    });
    c.bench_function("format user id", |b| {
        let mut string = String::new();
        let formatter = UserId::new(999_999_999_999_999_999)
            .expect("non zero")
            .mention();

        b.iter(|| format_id(&mut string, &formatter))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
