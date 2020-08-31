use criterion::{criterion_group, criterion_main, Criterion};
use std::{borrow::Cow, collections::{BTreeSet, HashSet}};

fn btreeset(set: &BTreeSet<Cow<'static, str>>) {
    set.get("!").unwrap();
}

fn hashset(set: &HashSet<Cow<'static, str>>) {
    set.get("!").unwrap();
}

fn vec(items: &[Cow<'static, str>]) {
    items.iter().find(|item| item.as_ref() == "!").unwrap();
}

fn vec_worst_case(items: &[Cow<'static, str>]) {
    items.iter().find(|item| item.as_ref() == "botname").unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("btreeset", |b| {
        let mut set = BTreeSet::new();
        set.insert(Cow::from("!"));
        set.insert(Cow::from("botname"));

        b.iter(|| {
            btreeset(&set);
        })
    });
    c.bench_function("hashset", |b| {
        let mut set = HashSet::new();
        set.insert(Cow::from("!"));
        set.insert(Cow::from("botname"));

        b.iter(|| {
            hashset(&set);
        })
    });
    c.bench_function("vec", |b| {
        let items = vec![Cow::from("!"), Cow::from("botname")];

        b.iter(|| {
            vec(&items);
        });
    });

    c.bench_function("vec_worst_case", |b| {
        let items = vec![Cow::from("!"), Cow::from("botname")];

        b.iter(|| {
            vec_worst_case(&items);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
