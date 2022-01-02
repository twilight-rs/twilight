use criterion::{criterion_group, criterion_main, Criterion};
use patricia_tree::PatriciaSet;
use std::{
    borrow::Cow,
    collections::{BTreeSet, HashSet},
};

fn btreeset(set: &BTreeSet<Cow<'static, str>>, needle: &str) {
    let start = needle.get(0..1).unwrap();
    set.range(Cow::from(start)..)
        .find(|item| needle.starts_with(item.as_ref()))
        .unwrap();
}

fn hashset(set: &HashSet<Cow<'static, str>>, needle: &str) {
    set.iter()
        .find(|item| needle.starts_with(item.as_ref()))
        .unwrap();
}

fn vec(items: &[Cow<'static, str>], needle: &str) {
    items
        .iter()
        .find(|item| needle.starts_with(item.as_ref()))
        .unwrap();
}

fn patricia_tree(items: &PatriciaSet, needle: &str) {
    items.get_longest_common_prefix(needle).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let commands = [
        "about",
        "coinflip",
        "help",
        "ping",
        "quote",
        "self_role",
        "uid",
        "commands",
        "emoji",
        "apexstats",
        "cat",
        "dog",
        "jumbo",
        "inf",
        "mwarn",
        "warn",
        "archive",
        "ban",
        "bean",
        "clean",
        "cleanban",
        "cleankick",
        "forceban",
        "kick",
        "mban",
        "mcleanban",
        "mkick",
        "munban",
        "mute",
        "nickname",
        "purge",
        "roles",
        "seen",
        "serverinfo",
        "slowmode",
        "tempban",
        "unban",
        "unmute",
        "userinfo",
        "verification",
        "remind",
        "configure",
        "disable",
    ];

    c.bench_function("c: btreeset", |b| {
        let set = commands.iter().map(|e| Cow::from(*e)).collect();

        b.iter(|| {
            for command in commands.iter() {
                btreeset(&set, command);
            }
        })
    });

    c.bench_function("c: hashset", |b| {
        let set = commands.iter().map(|e| Cow::from(*e)).collect();

        b.iter(|| {
            for command in commands.iter() {
                hashset(&set, command);
            }
        })
    });

    c.bench_function("c: vec", |b| {
        let items = commands.iter().map(|e| Cow::from(*e)).collect::<Vec<_>>();

        b.iter(|| {
            for command in commands.iter() {
                vec(&items, command);
            }
        });
    });

    c.bench_function("c: patricia_tree", |b| {
        let set = PatriciaSet::from_iter(commands.iter());

        b.iter(|| {
            for command in commands.iter() {
                patricia_tree(&set, command);
            }
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
