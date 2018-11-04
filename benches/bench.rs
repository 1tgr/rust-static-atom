extern crate criterion;
extern crate static_atom;

use std::fmt;

use criterion::{criterion_group, criterion_main, Bencher, Criterion, Fun};
use static_atom::{small, Big, Small};

fn match_keyword(s: &str) -> Result<Small, ()> {
    match s {
        "BTC-EUR" => Ok(small!("BTC-EUR")),
        "ETH-EUR" => Ok(small!("ETH-EUR")),
        "ETH-BTC" => Ok(small!("ETH-BTC")),
        _ => Err(()),
    }
}

fn test<F, T>(parser: F) -> impl FnMut(&mut Bencher, &(Result<usize, ()>, &str)) + 'static
where
    F: Fn(&str) -> Result<T, ()> + 'static,
    T: Into<usize> + PartialEq + fmt::Debug,
{
    move |b, &(expected, s)| {
        assert_eq!(expected, parser(s).map(Into::into));
        b.iter(|| {
            let s = criterion::black_box(s);
            let _ = criterion::black_box(parser(s));
        });
    }
}

fn bench(c: &mut Criterion) {
    let funs = || -> Vec<Fun<(Result<usize, ()>, &str)>> {
        vec![
            Fun::new("match_keyword", test(match_keyword)),
            Fun::new("trie_generated_small", test(str::parse::<Small>)),
            Fun::new("trie_generated_big", test(str::parse::<Big>)),
        ]
    };

    c.bench_functions("Valid 1", funs(), (Ok(0), "BTC-EUR"));
    c.bench_functions("Valid 2", funs(), (Ok(1), "ETH-EUR"));
    c.bench_functions("Valid 3", funs(), (Ok(2), "ETH-BTC"));
    c.bench_functions("Invalid empty", funs(), (Err(()), ""));
    c.bench_functions("Invalid too short", funs(), (Err(()), "ETH-"));
    c.bench_functions("Invalid too long", funs(), (Err(()), "ETH-EURzzz"));
    c.bench_functions("Invalid first char", funs(), (Err(()), "eTH-EUR"));
    c.bench_functions("Invalid last char", funs(), (Err(()), "ETH-EUr"));
}

criterion_group!(benches, bench);
criterion_main!(benches);
