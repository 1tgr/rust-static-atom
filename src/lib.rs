#![feature(plugin)]
#![feature(test)]
#![plugin(phf_macros)]

extern crate phf;
extern crate test;

use std::mem;
use std::num::NonZeroU8;

use test::Bencher;

fn read<'a, T: Eq>(s: &mut &'a [T]) -> Option<&'a T> {
    if let Some(b) = s.get(0) {
        *s = &s[1..];
        Some(b)
    } else {
        None
    }
}

fn expect<T: Eq>(s: &mut &[T], b: &T) -> Option<()> {
    if read(s)? == b {
        Some(())
    } else {
        None
    }
}

fn expect_str<T: Eq>(s: &mut &[T], s2: &[T]) -> Option<()> {
    for b in s2 {
        expect(s, b)?;
    }

    Some(())
}

fn expect_str2(s: &mut &[u8], s2: &[u8; 2]) -> Option<()> {
    let s_2: &[u16] = unsafe { mem::transmute(*s) };
    let s2_2: &u16 = unsafe { mem::transmute(s2) };
    if let Some(b_2) = s_2.get(0) {
        if b_2 == s2_2 {
            *s = &s[2..];
            return Some(());
        }
    }

    None
}

fn expect_str4(s: &mut &[u8], s2: &[u8; 4]) -> Option<()> {
    let s_4: &[u32] = unsafe { mem::transmute(*s) };
    let s2_4: &u32 = unsafe { mem::transmute(s2) };
    if let Some(b_4) = s_4.get(0) {
        if b_4 == s2_4 {
            *s = &s[4..];
            return Some(());
        }
    }

    None
}

fn expect_eof<T>(s: &[T]) -> Option<()> {
    if s.is_empty() {
        Some(())
    } else {
        None
    }
}

const ONE: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(1) };
const TWO: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(2) };
const THREE: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(3) };

fn match_keyword(s: &str) -> Option<NonZeroU8> {
    match s {
        "BTC-EUR" => Some(ONE),
        "ETH-EUR" => Some(TWO),
        "ETH-BTC" => Some(THREE),
        _ => None,
    }
}

fn trie_u8(s: &str) -> Option<NonZeroU8> {
    let mut s = s.as_bytes();
    if s.len() != 7 {
        return None;
    }

    match read(&mut s)? {
        b'B' => {
            expect_str(&mut s, b"TC-EUR")?;
            expect_eof(s)?;
            Some(ONE)
        }

        b'E' => {
            expect_str(&mut s, b"TH-")?;
            match read(&mut s)? {
                b'E' => {
                    expect_str(&mut s, b"UR")?;
                    expect_eof(s)?;
                    Some(TWO)
                }

                b'B' => {
                    expect_str(&mut s, b"TC")?;
                    expect_eof(s)?;
                    Some(THREE)
                }

                _ => None,
            }
        }

        _ => None,
    }
}

fn trie_u8_u32(s: &str) -> Option<NonZeroU8> {
    let mut s = s.as_bytes();
    if s.len() != 7 {
        return None;
    }

    match read(&mut s)? {
        b'B' => {
            expect_str4(&mut s, b"TC-E")?;
            expect_str2(&mut s, b"UR")?;
            expect_eof(s)?;
            Some(ONE)
        }

        b'E' => {
            expect_str2(&mut s, b"TH")?;
            expect(&mut s, &b'-')?;
            match read(&mut s)? {
                b'E' => {
                    expect_str2(&mut s, b"UR")?;
                    expect_eof(s)?;
                    Some(TWO)
                }

                b'B' => {
                    expect_str2(&mut s, b"TC")?;
                    expect_eof(s)?;
                    Some(THREE)
                }

                _ => None,
            }
        }

        _ => None,
    }
}

fn trie_u32_u8(s: &str) -> Option<NonZeroU8> {
    let mut s = s.as_bytes();
    if s.len() != 7 {
        return None;
    }

    if let Some(()) = expect_str4(&mut s, b"BTC-") {
        expect_str2(&mut s, b"EU")?;
        expect(&mut s, &b'R')?;
        expect_eof(s)?;
        Some(ONE)
    } else if let Some(()) = expect_str4(&mut s, b"ETH-") {
        if let Some(()) = expect_str2(&mut s, b"EU") {
            expect(&mut s, &b'R')?;
            expect_eof(s)?;
            Some(TWO)
        } else if let Some(()) = expect_str2(&mut s, b"BT") {
            expect(&mut s, &b'C')?;
            expect_eof(s)?;
            Some(THREE)
        } else {
            None
        }
    } else {
        None
    }
}

fn phf(s: &str) -> Option<NonZeroU8> {
    static TOKENS: phf::Map<&'static str, NonZeroU8> = phf_map! {
        "BTC-EUR" => ONE,
        "ETH-EUR" => TWO,
        "ETH-BTC" => THREE,
    };

    TOKENS.get(s).cloned()
}

fn assert_eq<T: std::fmt::Debug + Eq>(a: T, b: T) {
    assert_eq!(a, b);
    test::black_box(a);
    test::black_box(b);
}

fn test(parser: impl Fn(&str) -> Option<NonZeroU8>) {
    assert_eq(Some(ONE), parser("BTC-EUR"));
    assert_eq(Some(TWO), parser("ETH-EUR"));
    assert_eq(Some(THREE), parser("ETH-BTC"));
    assert_eq(None, parser(""));
    assert_eq(None, parser("ETH-"));
    assert_eq(None, parser("ETH-EURzzz"));
}

#[bench]
fn bench_match_keyword(b: &mut Bencher) {
    b.iter(|| test(match_keyword));
}

#[bench]
fn bench_trie_u8(b: &mut Bencher) {
    b.iter(|| test(trie_u8));
}

#[bench]
fn bench_trie_u8_u32(b: &mut Bencher) {
    b.iter(|| test(trie_u8_u32));
}

#[bench]
fn bench_trie_u32_u8(b: &mut Bencher) {
    b.iter(|| test(trie_u32_u8));
}

#[bench]
fn bench_phf(b: &mut Bencher) {
    b.iter(|| test(phf));
}
