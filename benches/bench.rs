extern crate static_atom;

#[macro_use]
extern crate criterion;

use std::num::NonZeroU8;

use criterion::Criterion;
use static_atom::*;

fn test(parser: impl Fn(&str) -> Option<NonZeroU8>) {
    let parser = |s| {
        criterion::black_box(parser(criterion::black_box(s)))
    };

    assert_eq!(Some(1), parser("BTC-EUR").map(NonZeroU8::get));
    assert_eq!(Some(2), parser("ETH-EUR").map(NonZeroU8::get));
    assert_eq!(Some(3), parser("ETH-BTC").map(NonZeroU8::get));
    assert_eq!(None, parser(""));
    assert_eq!(None, parser("ETH-"));
    assert_eq!(None, parser("ETH-EURzzz"));
}

fn bench(c: &mut Criterion) {
    c.bench_function("match_keyword", |b| b.iter(|| test(match_keyword)));
    c.bench_function("phf", |b| b.iter(|| test(phf)));
    c.bench_function("static_map", |b| b.iter(|| test(static_map)));
    c.bench_function("trie_u8", |b| b.iter(|| test(trie_u8)));
    c.bench_function("trie_u8_u32", |b| b.iter(|| test(trie_u8_u32)));
    c.bench_function("trie_u32_u8", |b| b.iter(|| test(trie_u32_u8)));
    c.bench_function("trie_generated_small", |b| b.iter(|| test(small::from_str)));
    c.bench_function("trie_generated_big", |b| b.iter(|| test(big::from_str)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
