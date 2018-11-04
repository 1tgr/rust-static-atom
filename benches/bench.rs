extern crate criterion;
extern crate static_atom;

use std::fmt;

use criterion::{criterion_group, criterion_main, Criterion};
use static_atom::{small, Big, Small};

fn match_keyword(s: &str) -> Result<Small, ()> {
    match s {
        "BTC-EUR" => Ok(small!("BTC-EUR")),
        "ETH-EUR" => Ok(small!("ETH-EUR")),
        "ETH-BTC" => Ok(small!("ETH-BTC")),
        _ => Err(()),
    }
}

fn test<T>(parser: impl Fn(&str) -> Result<T, ()>)
where
    T: Into<usize> + PartialEq + fmt::Debug,
{
    let parser = |s| criterion::black_box(parser(criterion::black_box(s)));

    assert_eq!(Ok(0), parser("BTC-EUR").map(Into::into));
    assert_eq!(Ok(1), parser("ETH-EUR").map(Into::into));
    assert_eq!(Ok(2), parser("ETH-BTC").map(Into::into));
    assert_eq!(Err(()), parser(""));
    assert_eq!(Err(()), parser("ETH-"));
    assert_eq!(Err(()), parser("ETH-EURzzz"));
}

fn bench(c: &mut Criterion) {
    c.bench_function("match_keyword", |b| b.iter(|| test(match_keyword)));
    c.bench_function("trie_generated_small", |b| b.iter(|| test(str::parse::<Small>)));
    c.bench_function("trie_generated_big", |b| b.iter(|| test(str::parse::<Big>)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
