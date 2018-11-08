extern crate criterion;
extern crate crypto_example;

use std::fmt;

use criterion::{criterion_group, criterion_main, Bencher, Criterion, Fun};
use crypto_example::{small, Big, Convention, ConventionVisitor, Small};

fn match_keyword(s: &str) -> Result<Small, ()> {
    match s {
        "BTC-EUR" => Ok(small!("BTC-EUR")),
        "BTC-USDC" => Ok(small!("BTC-USDC")),
        "ETH-EUR" => Ok(small!("ETH-EUR")),
        "ETH-BTC" => Ok(small!("ETH-BTC")),
        _ => Err(()),
    }
}

fn bench_consts(c: &mut Criterion) {
    fn test_quote_increment(b: &mut Bencher, input: &(f64, Small)) {
        struct Visitor<'a> {
            b: &'a mut Bencher,
            expected: f64
        };

        impl<'a> ConventionVisitor for Visitor<'a> {
            type Value = ();

            fn visit<C: Convention>(self) -> () {
                assert_eq!(self.expected, crypto_example::quote_increment::<C>());
                self.b.iter(|| {
                    criterion::black_box(crypto_example::quote_increment::<C>())
                })
            }
        }

        let &(expected, atom) = input;
        atom.visit_convention(Visitor { b, expected })
    }

    c.bench_function_over_inputs(
        "quote_increment",
        test_quote_increment,
        vec![
            (0.01, small!("BTC-EUR")),
            (0.01, small!("BTC-USDC")),
            (0.01, small!("ETH-EUR")),
            (0.00001, small!("ETH-BTC")),
        ],
    );
}

fn bench_parse(c: &mut Criterion) {
    fn test<F, T>(parser: F) -> impl FnMut(&mut Bencher, &(Result<usize, ()>, &str)) + 'static
    where
        F: Fn(&str) -> Result<T, ()> + 'static,
        T: Into<usize> + PartialEq + fmt::Debug,
    {
        move |b, &(expected, s)| {
            assert_eq!(expected, parser(s).map(Into::into));

            let s = s.to_owned();
            b.iter(|| {
                let s = criterion::black_box(&s);
                let _ = criterion::black_box(parser(s));
            });
        }
    }

    let funs = || -> Vec<Fun<(Result<usize, ()>, &str)>> {
        vec![
            Fun::new("match_keyword", test(match_keyword)),
            Fun::new("trie_generated_small", test(str::parse::<Small>)),
            Fun::new("trie_generated_big", test(str::parse::<Big>)),
        ]
    };

    c.bench_functions("Valid 1", funs(), (Ok(0), "BTC-EUR"));
    c.bench_functions("Valid 2", funs(), (Ok(1), "BTC-USDC"));
    c.bench_functions("Valid 3", funs(), (Ok(2), "ETH-EUR"));
    c.bench_functions("Valid 4", funs(), (Ok(3), "ETH-BTC"));
    c.bench_functions("Invalid empty", funs(), (Err(()), ""));
    c.bench_functions("Invalid too short", funs(), (Err(()), "ETH-"));
    c.bench_functions("Invalid too long", funs(), (Err(()), "ETH-EURzzz"));
    c.bench_functions("Invalid first char", funs(), (Err(()), "eTH-EUR"));
    c.bench_functions("Invalid last char", funs(), (Err(()), "ETH-EUr"));
}

criterion_group!(benches, bench_consts, bench_parse);
criterion_main!(benches);
