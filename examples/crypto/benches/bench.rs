extern crate criterion;
extern crate crypto_example;
extern crate static_atom;
extern crate try_from;

use std::fmt;

use criterion::{criterion_group, criterion_main, Bencher, Criterion, Fun};
use crypto_example::{small, small_type, Convention, ConventionVisitor, Price, PriceMapping};
use crypto_example::atoms::{Big, Small, TypedSmallMap};
use static_atom::TypedAtomMap;
use try_from::TryFrom;

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
    fn test(b: &mut Bencher, input: &(i32, Small)) {
        struct Visitor<'a> {
            b: &'a mut Bencher,
            expected: i32,
        };

        impl<'a> ConventionVisitor for Visitor<'a> {
            type Value = ();

            fn visit<C: Convention>(self) -> () {
                let Visitor { b, expected } = self;
                assert_eq!(expected, crypto_example::price_digits::<C>());
                b.iter(|| criterion::black_box(crypto_example::price_digits::<C>()))
            }
        }

        let &(expected, atom) = input;
        atom.visit_convention(Visitor { b, expected })
    }

    c.bench_function_over_inputs(
        "price_digits",
        test,
        vec![
            (2, small!("BTC-EUR")),
            (2, small!("BTC-USDC")),
            (2, small!("ETH-EUR")),
            (5, small!("ETH-BTC")),
        ],
    );
}

fn bench_typed_atom_map(c: &mut Criterion) {
    fn test<C: Convention + Eq + Copy + 'static>(b: &mut Bencher, _: &()) {
        let mut m = TypedSmallMap::<PriceMapping>::new();
        let price1 = Price::try_from(6000.0).unwrap();
        let price2 = Price::try_from(6001.0).unwrap();
        b.iter(move || {
            assert_eq!(None, m.insert::<C>(price1));
            assert_eq!(Some(price1), m.get::<C>().cloned());
            assert_eq!(Some(price1), m.insert::<C>(price2));
            assert_eq!(Some(price2), m.remove::<C>());
        });
    }

    let funs: Vec<Fun<()>> = vec![
        Fun::new("BTC-EUR", test::<small_type!("BTC-EUR")>),
        Fun::new("BTC-USDC", test::<small_type!("BTC-USDC")>),
        Fun::new("ETH-EUR", test::<small_type!("ETH-EUR")>),
        Fun::new("ETH-BTC", test::<small_type!("ETH-BTC")>),
    ];

    c.bench_functions("TypedAtomMap", funs, ());
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

criterion_group!(benches, bench_consts, bench_parse, bench_typed_atom_map);
criterion_main!(benches);
