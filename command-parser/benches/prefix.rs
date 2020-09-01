use criterion::{criterion_group, criterion_main, Criterion};
use std::{borrow::Cow, collections::{BTreeSet, HashSet}};
use patricia_tree::PatriciaSet;

fn btreeset(set: &BTreeSet<Cow<'static, str>>) {
    set.iter().find(|item| ("!command").starts_with(item.as_ref())).unwrap();
}

fn hashset(set: &HashSet<Cow<'static, str>>) {
    set.iter().find(|item| ("!command").starts_with(item.as_ref())).unwrap();
}

fn vec(items: &[Cow<'static, str>]) {
    items.iter().find(|item| ("!command").starts_with(item.as_ref())).unwrap();
}

fn vec_worst_case(items: &[Cow<'static, str>]) {
    items.iter().find(|item| ("!command").starts_with(item.as_ref())).unwrap();
}

fn patricia_tree(items: &PatriciaSet) {
    let _ = items.get_longest_common_prefix("!command").is_some();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("p: btreeset", |b| {
        let mut set = BTreeSet::new();
        set.insert(Cow::from("!"));
        set.insert(Cow::from("botname"));

        b.iter(|| {
            btreeset(&set);
        })
    });
    c.bench_function("p: hashset", |b| {
        let mut set = HashSet::new();
        set.insert(Cow::from("!"));
        set.insert(Cow::from("botname"));

        b.iter(|| {
            hashset(&set);
        })
    });
    c.bench_function("p: vec", |b| {
        let items = vec![Cow::from("!"), Cow::from("botname")];

        b.iter(|| {
            vec(&items);
        });
    });

    c.bench_function("p: vec_worst_case", |b| {
        let items = vec![Cow::from("!"), Cow::from("botname")];

        b.iter(|| {
            vec_worst_case(&items);
        });
    });

    c.bench_function("p: patricia_tree", |b| {
        let mut set = PatriciaSet::new();
        set.insert("!");
        set.insert("botname");

        b.iter(|| {
            patricia_tree(&set);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
