#![feature(plugin)]
#![plugin(phf_macros)]

extern crate phf;

#[macro_use]
extern crate static_map;

#[macro_use]
extern crate static_map_macros;

use std::mem;
use std::num::NonZeroU8;

#[inline]
fn read<'a, T>(s: &mut &'a [T]) -> Option<&'a T> {
    if let Some(b) = s.first() {
        *s = &s[1..];
        Some(b)
    } else {
        None
    }
}

#[inline]
fn expect<'a, T: PartialEq>(s: &mut &[T], value: &T) -> Option<()> {
    if let Some(b) = read(s) {
        if b == value {
            return Some(());
        }
    }

    None
}

trait ArrayExpect<T> {
    fn expect(s: &mut &[T], a: Self) -> Option<()>;
}

impl<A, T> ArrayExpect<T> for &A
where
    A: ArrayExpect<T> + Copy,
{
    fn expect(s: &mut &[T], a: &A) -> Option<()> {
        ArrayExpect::expect(s, *a)
    }
}

impl ArrayExpect<u8> for [u8; 1] {
    #[inline]
    fn expect(s: &mut &[u8], a: [u8; 1]) -> Option<()> {
        expect(s, &a[0])
    }
}

impl ArrayExpect<u8> for [u8; 2] {
    #[inline]
    fn expect(s: &mut &[u8], a: [u8; 2]) -> Option<()> {
        let mut s2: &[u16] = unsafe { mem::transmute(*s) };
        let a2: u16 = unsafe { mem::transmute(a) };
        expect(&mut s2, &a2)?;
        *s = &s[2..];
        Some(())
    }
}

impl ArrayExpect<u8> for [u8; 3] {
    #[inline]
    fn expect(s: &mut &[u8], a: [u8; 3]) -> Option<()> {
        ArrayExpect::expect(s, [a[0], a[1]])?;
        ArrayExpect::expect(s, [a[2]])?;
        Some(())
    }
}

impl ArrayExpect<u8> for [u8; 4] {
    #[inline]
    fn expect(s: &mut &[u8], a: [u8; 4]) -> Option<()> {
        let mut s4: &[u32] = unsafe { mem::transmute(*s) };
        let a4: u32 = unsafe { mem::transmute(a) };
        expect(&mut s4, &a4)?;
        *s = &s[4..];
        Some(())
    }
}

impl ArrayExpect<u8> for [u8; 5] {
    #[inline]
    fn expect(s: &mut &[u8], a: [u8; 5]) -> Option<()> {
        ArrayExpect::expect(s, [a[0], a[1], a[2], a[3]])?;
        ArrayExpect::expect(s, [a[4]])?;
        Some(())
    }
}

impl ArrayExpect<u8> for [u8; 6] {
    #[inline]
    fn expect(s: &mut &[u8], a: [u8; 6]) -> Option<()> {
        ArrayExpect::expect(s, [a[0], a[1], a[2], a[3]])?;
        ArrayExpect::expect(s, [a[4], a[5]])?;
        Some(())
    }
}

impl ArrayExpect<u8> for [u8; 7] {
    #[inline]
    fn expect(s: &mut &[u8], a: [u8; 7]) -> Option<()> {
        ArrayExpect::expect(s, [a[0], a[1], a[2], a[3]])?;
        ArrayExpect::expect(s, [a[4], a[5]])?;
        ArrayExpect::expect(s, [a[6]])?;
        Some(())
    }
}

include!(concat!(env!("OUT_DIR"), "/atoms.rs"));

const ONE: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(1) };
const TWO: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(2) };
const THREE: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(3) };

pub fn match_keyword(s: &str) -> Option<NonZeroU8> {
    match s {
        "BTC-EUR" => Some(ONE),
        "ETH-EUR" => Some(TWO),
        "ETH-BTC" => Some(THREE),
        _ => None,
    }
}

pub fn trie_u8(s: &str) -> Option<NonZeroU8> {
    let mut s = s.as_bytes();
    if s.len() != 7 {
        return None;
    }

    match read(&mut s)? {
        b'B' => {
            ArrayExpect::expect(&mut s, b"TC-EUR")?;
            Some(ONE)
        }

        b'E' => {
            ArrayExpect::expect(&mut s, b"TH-")?;
            match read(&mut s)? {
                b'E' => {
                    ArrayExpect::expect(&mut s, b"UR")?;
                    Some(TWO)
                }

                b'B' => {
                    ArrayExpect::expect(&mut s, b"TC")?;
                    Some(THREE)
                }

                _ => None,
            }
        }

        _ => None,
    }
}

pub fn trie_u8_u32(s: &str) -> Option<NonZeroU8> {
    let mut s = s.as_bytes();
    if s.len() != 7 {
        return None;
    }

    match read(&mut s)? {
        b'B' => {
            ArrayExpect::expect(&mut s, b"TC-E")?;
            ArrayExpect::expect(&mut s, b"UR")?;
            Some(ONE)
        }

        b'E' => {
            ArrayExpect::expect(&mut s, b"TH")?;
            ArrayExpect::expect(&mut s, b"-")?;
            match read(&mut s)? {
                b'E' => {
                    ArrayExpect::expect(&mut s, b"UR")?;
                    Some(TWO)
                }

                b'B' => {
                    ArrayExpect::expect(&mut s, b"TC")?;
                    Some(THREE)
                }

                _ => None,
            }
        }

        _ => None,
    }
}

pub fn trie_u32_u8(s: &str) -> Option<NonZeroU8> {
    let mut s = s.as_bytes();
    if s.len() != 7 {
        return None;
    }

    if let Some(()) = ArrayExpect::expect(&mut s, b"BTC-") {
        ArrayExpect::expect(&mut s, b"EU")?;
        ArrayExpect::expect(&mut s, b"R")?;
        Some(ONE)
    } else if let Some(()) = ArrayExpect::expect(&mut s, b"ETH-") {
        if let Some(()) = ArrayExpect::expect(&mut s, b"EU") {
            ArrayExpect::expect(&mut s, b"R")?;
            Some(TWO)
        } else if let Some(()) = ArrayExpect::expect(&mut s, b"BT") {
            ArrayExpect::expect(&mut s, b"C")?;
            Some(THREE)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn phf(s: &str) -> Option<NonZeroU8> {
    static TOKENS: phf::Map<&'static str, NonZeroU8> = phf_map! {
        "BTC-EUR" => ONE,
        "ETH-EUR" => TWO,
        "ETH-BTC" => THREE,
    };

    TOKENS.get(s).cloned()
}

pub fn static_map(s: &str) -> Option<NonZeroU8> {
    static TOKENS: static_map::Map<&'static str, NonZeroU8> = static_map! {
        Default: ONE,
        "BTC-EUR" => ONE,
        "ETH-EUR" => TWO,
        "ETH-BTC" => THREE,
    };

    TOKENS.get(s).cloned()
}
